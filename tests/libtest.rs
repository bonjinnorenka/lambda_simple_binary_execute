#[cfg(test)]
mod tests {
    use lambda_simple_binary_execute::*;
    #[test]
    fn test_executor() {
        let con = Content {
            command: Some(env!("CARGO_BIN_EXE_hello_world_test").to_string()),
            args: None,
            stdin: None,
            complement_aws_path: Some(false),
        };
        let output_expected = StdoutErr {
            stdout: "Hello, world!\n".to_string(),
            stderr: "".to_string(),
        };
        let response = executor(con).unwrap();
        assert_eq!(response.body, serde_json::to_string(&output_expected).unwrap());
    }

    #[test]
    fn test_main_panic() {
        let con = Content {
            command: Some(env!("CARGO_BIN_EXE_hello_world_test").to_string()),
            args: Some(vec!["--panic".to_string()]),
            stdin: None,
            complement_aws_path: Some(false),
        };
        let response = executor(con);
        assert!(response.is_err());
    }

    #[test]
    fn test_main_use_stdin() {
        let input = "Test input";
        let con = Content {
            command: Some(env!("CARGO_BIN_EXE_hello_world_test").to_string()),
            args: Some(vec!["--use-stdin".to_string()]),
            stdin: Some(input.to_string()),
            complement_aws_path: Some(false),
        };
        let output_expected = StdoutErr {
            stdout: input.to_string() + "\n",
            stderr: "".to_string(),
        };
        let response = executor(con).unwrap();
        assert_eq!(response.body, serde_json::to_string(&output_expected).unwrap());
    }
}