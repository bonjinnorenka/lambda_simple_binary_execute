# lambda_simple_binary_execute

Simple binary executor for AWS Lambda.

x86_64 and arm64 binaries are available.

# Usage

1. Download the binary for lambda runtime architecture

[Download x86_64 binary](https://github.com/bonjinnorenka/lambda_simple_binary_execute/releases/latest/download/lambda_simple_binary_execute-x86_64)

[Download arm64 binary](https://github.com/bonjinnorenka/lambda_simple_binary_execute/releases/latest/download/lambda_simple_binary_execute-aarch64)

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
