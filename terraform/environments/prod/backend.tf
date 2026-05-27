terraform {
  backend "s3" {
    bucket       = "slack-lab-bot-terraform-state"
    key          = "prod/terraform.tfstate"
    acl          = "private"
    encrypt      = true
    use_lockfile = true
  }
}
