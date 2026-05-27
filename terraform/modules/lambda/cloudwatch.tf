resource "aws_cloudwatch_log_group" "lambda" {
  name              = "/aws/lambda/${aws_lambda_function.slack_lab_bot.function_name}"
  retention_in_days = var.log_retention_days
}
