variable "lambda_timeout" {
  description = "Timeout for the Lambda function in seconds"
  type        = number
}

variable "lambda_memory_size" {
  description = "Memory size for the Lambda function in MB"
  type        = number
}

variable "slack_bot_token" {
  description = "Slack Bot Token"
  type        = string
  sensitive   = true
}

variable "weekly_report_channel_id" {
  description = "Slack channel ID for weekly reports"
  type        = string
}

variable "student_ids" {
  description = "Comma-separated list of Slack student user IDs"
  type        = string
}

variable "schedule_expression" {
  description = "EventBridge schedule expression for triggering the Lambda (e.g. \"cron(0 0 ? * MON *)\")"
  type        = string
}

variable "log_retention_days" {
  description = "CloudWatch log retention period in days"
  type        = number
}
