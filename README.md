# lambda_simple_binary_execute

Simple binary executor for AWS Lambda.

x86_64 and aarch64 binaries are available.

# Features

- Light weight
- No dependencies

# How fast is it?

| Metric          | x86_64 first | x86_64 | aarch64 first | aarch64 |
|-----------------|--------------|--------|---------------|---------|
| init time [ms]  | 33.36        | 0      | 26.52         | 0       |
| duration [ms]   | 67.2         | 3~22   | 47.12         | 3~21    |
| max memory [MB] | 17           | 18     | 17            | 18      |

## Measurement Conditions

The measurements were taken under the following conditions:

- AWS Region: ap-northeast-1
- Memory: 128MB
- Execution: Using environment variable `LSBE_COMMAND`
- Command: `hello_world_test` without any arguments

# Usage

1. Download the binary for lambda runtime architecture

[Download x86_64 binary](https://github.com/bonjinnorenka/lambda_simple_binary_execute/releases/latest/download/lambda_simple_binary_execute-x86_64)

[Download aarch64 binary](https://github.com/bonjinnorenka/lambda_simple_binary_execute/releases/latest/download/lambda_simple_binary_execute-aarch64)

1. Rename the binary to `bootstrap` and make it executable

```bash
mv lambda_simple_binary_execute-x86_64 bootstrap
```

3. Create a zip file with the binary

```bash
zip lambda.zip bootstrap YOUR_BINARY
```

4. Upload the zip file to AWS Lambda
Note: Please use Amazon Linux 2 as the runtime.

5. Set Environment Variables or Event Payload

**Environment Variables**

`LSBE_COMMAND` - Command to execute

**Event Payload**

```json
{
  "command": "ls",
  "args": ["-l"]
}
```
