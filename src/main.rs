use lambda_simple_binary_execute::{Content, executor, next_request, post_error, post_response};

#[tokio::main]
async fn main() {
    loop {
        let payload = next_request().await;
        let content = serde_json::from_str::<Content>(&payload.body).unwrap();
        let response = executor(content);
        if response.is_err() {
            let error = format!("{{\"errorType\": \"executor_error\", \"errorMessage\": \"Failed to execute command\"}}");
            post_error(&payload, error).await;
            continue;
        }
        post_response(&payload, serde_json::to_string(&response.unwrap()).unwrap()).await;
    }
}
