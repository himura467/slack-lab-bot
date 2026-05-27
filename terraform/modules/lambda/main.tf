resource "aws_lambda_function" "slack_lab_bot" {
  function_name    = "slack-lab-bot-lambda-function"
  role             = aws_iam_role.lambda.arn
  architectures    = ["x86_64"]
  filename         = "../../../slack-lab-bot.zip"
  source_code_hash = filebase64sha256("../../../slack-lab-bot.zip")
  handler          = "bootstrap"
  runtime          = "provided.al2023"
  timeout          = var.lambda_timeout
  memory_size      = var.lambda_memory_size
  environment {
    variables = {
      SLACK_BOT_TOKEN          = var.slack_bot_token
      WEEKLY_REPORT_CHANNEL_ID = var.weekly_report_channel_id
      STUDENT_IDS              = var.student_ids
    }
  }
}
