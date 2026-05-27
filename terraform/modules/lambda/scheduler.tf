resource "aws_cloudwatch_event_rule" "weekly" {
  name                = "slack-lab-bot-weekly"
  schedule_expression = var.schedule_expression
}

resource "aws_cloudwatch_event_target" "weekly" {
  rule      = aws_cloudwatch_event_rule.weekly.name
  target_id = aws_lambda_function.slack_lab_bot.function_name
  arn       = aws_lambda_function.slack_lab_bot.arn
}

resource "aws_lambda_permission" "allow_eventbridge" {
  statement_id  = "AllowEventBridgeInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.slack_lab_bot.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.weekly.arn
}
