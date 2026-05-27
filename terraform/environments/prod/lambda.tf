module "lambda" {
  source                   = "../../modules/lambda"
  lambda_timeout           = 30
  lambda_memory_size       = 128
  slack_bot_token          = var.slack_bot_token
  weekly_report_channel_id = var.weekly_report_channel_id
  student_ids              = var.student_ids
  schedule_expression      = "cron(0 12 ? * MON *)"
  log_retention_days       = 30
}
