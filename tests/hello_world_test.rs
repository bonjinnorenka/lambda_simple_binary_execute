#[cfg(test)]
mod tests {
    use std::{io::Write, process::{Command, Stdio}};

    #[test]
    fn test_main_hello_world() {
        let output = Command::new(env!("CARGO_BIN_EXE_hello_world_test"))
            .arg("hello_world_test")
            .output()
            .expect("Failed to execute process");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Hello, world!");
    }

    #[test]
    fn test_main_panic() {
        let output = Command::new(env!("CARGO_BIN_EXE_hello_world_test"))
            .arg("--panic")
            .output()
            .expect("Failed to execute process");
        assert!(output.status.code() == Some(101));
    }

    #[test]
    fn test_main_use_stdin() {
        let input = "Test input";
        let mut child = Command::new(env!("CARGO_BIN_EXE_hello_world_test"))
            .arg("--use-stdin")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute process");

        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");

        let output = child.wait_with_output().expect("Failed to read stdout");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), input);
    }
}