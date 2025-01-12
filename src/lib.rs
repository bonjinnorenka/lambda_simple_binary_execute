use reqwest;
use std::{io::Write, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone)]
pub struct Payload {
    pub body: String,
    pub request_id: String,
    pub function_arn: String,
    pub deadline: String,
    pub trace_id: String,
}

impl Payload {
    pub fn new(body: String, request_id: String, function_arn: String, deadline: String, trace_id: String) -> Payload {
        Payload {
            body,
            request_id,
            function_arn,
            deadline,
            trace_id,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Response {
    pub body: String,
}

impl Response {
    pub fn new(body: String) -> Response {
        Response {
            body,
        }
    }
}

fn get_runtime_endpoint() -> String {
    std::env::var("AWS_LAMBDA_RUNTIME_API").expect("AWS_LAMBDA_RUNTIME_API is not set")
}

pub async fn init_error(error: String) {
    let url = format!("http://{}/2018-06-01/runtime/init/error", get_runtime_endpoint());
    let client = reqwest::Client::new();
    let response = client.post(&url)
        .body(error)
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
}

pub async fn post_response(payload: &Payload, response: String) {
    let url = format!("http://{}/2018-06-01/runtime/invocation/{}/response", get_runtime_endpoint(), &payload.request_id);
    let client = reqwest::Client::new();
    if let Ok(response) = client.post(&url).body(response).send().await{
        assert!(response.status().is_success());
    }
    else{
        let error = format!("{{\"errorType\": \"response_error\", \"errorMessage\": \"Failed to send response\"}}");
        post_error(&payload, error).await;
    }
}

pub async fn post_error(payload: &Payload, error: String) {
    let url = format!("http://{}/2018-06-01/runtime/invocation/{}/error", get_runtime_endpoint(), &payload.request_id);
    let client = reqwest::Client::new();
    let response = client.post(&url)
        .body(error)
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
}

pub async fn next_request() -> Payload {
    let url = format!("http://{}/2018-06-01/runtime/invocation/next", get_runtime_endpoint());
    let response = reqwest::get(&url).await.unwrap();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap_or("".to_string());
    let empty_header_value = reqwest::header::HeaderValue::from_static("");
    let request_id = headers.get("Lambda-Runtime-Aws-Request-Id").unwrap().to_str().expect("Failed to get request id");
    let function_arn = headers.get("Lambda-Runtime-Invoked-Function-Arn").unwrap_or(&empty_header_value).to_str().unwrap_or("arn:aws:lambda:us-west-2:123456789012:function:custom-runtime");
    let deadline = headers.get("Lambda-Runtime-Deadline-Ms").unwrap_or(&empty_header_value).to_str().unwrap_or("1600000000000");
    let trace_id = headers.get("Lambda-Runtime-Trace-Id").unwrap_or(&empty_header_value).to_str().unwrap_or("1-23456789-123456789012345678901234567890");
    let payload = Payload::new(body, request_id.to_string(), function_arn.to_string(), deadline.to_string(), trace_id.to_string());
    payload
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Content {
    pub command: Option<String>,//if empty, find executable in path
    pub args: Option<Vec<String>>,
    pub stdin: Option<String>,
    pub complement_aws_path: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StdoutErr {
    pub stdout: String,
    pub stderr: String,
}

pub fn executor(con:Content) -> Result<Response, Box<dyn std::error::Error>> {
    let command = if con.command.is_some() {
        if con.complement_aws_path.unwrap_or(true) && !con.command.clone().unwrap().contains("/") {
            std::env::var("LAMBDA_TASK_ROOT").unwrap_or("".to_string()) + "/" + &con.command.unwrap()
        } else {
            con.command.unwrap()
        }
    } else {
        let lsbe_command = std::env::var("LSBE_COMMAND").unwrap_or("".to_string());
        if lsbe_command.len() > 0 {
            if con.complement_aws_path.unwrap_or(true) && !lsbe_command.contains("/") {
                std::env::var("LAMBDA_TASK_ROOT").unwrap_or("".to_string()) + "/" + &lsbe_command
            } else {
                lsbe_command
            }
        }
        else{
            let current_dir = if con.complement_aws_path.unwrap_or(true) {
                std::env::var("LAMBDA_TASK_ROOT")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| std::env::current_dir().expect("Failed to get current directory"))
            }
            else{
                std::env::current_dir().expect("Failed to get current directory")
            };
            let command = current_dir.iter()
                .find_map(|entry| {
                    let path = current_dir.join(entry);
                    if path.is_file() {
                        #[cfg(unix)]
                        {
                            if path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                                if path.to_str() == Some(std::env::current_exe().unwrap().to_str().unwrap()) {
                                    return None;
                                }
                                return path.to_str().map(|s| s.to_string());
                            }
                        }
                        #[cfg(windows)]
                        {
                            if path.extension().map(|ext| ext.eq_ignore_ascii_case("exe")).unwrap_or(false) {
                                return path.to_str().map(|s| s.to_string());
                            }
                        }
                    }
                    None
                })
                .expect(format!("Failed to find executable in {:?}", current_dir).as_str());
            command
        }
    };
    let args = con.args.unwrap_or(vec![]);
    let stdin_content = con.stdin.unwrap_or("".to_string());
    let mut child = std::process::Command::new(command);
    for arg in args.iter() {
        child.arg(arg);
    }
    if stdin_content.len() > 0 {
        child.stdin(std::process::Stdio::piped());
    }
    child.stdout(std::process::Stdio::piped());
    let mut child = child.spawn().expect("Failed to spawn child process");
    if stdin_content.len() > 0 {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(stdin_content.as_bytes()).expect("Failed to write to stdin");
    }
    let output = child.wait_with_output().expect("Failed to wait for child process");
    let output_res = StdoutErr {
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    };
    let response = Response::new(serde_json::to_string(&output_res).unwrap());

    if output.status.success() {
        Ok(response)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, response.body)))
    }
}