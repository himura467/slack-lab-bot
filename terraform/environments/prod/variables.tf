variable "aws_region" {
  description = "AWS region"
  type        = string
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
