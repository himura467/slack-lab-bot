# Terraform

## Before Init

Run this:

```sh
OP_VAULT_NAME="Slack Lab Bot" OP_APP_ENV="Production" op run --env-file provider.env -- aws s3api create-bucket --bucket slack-lab-bot-terraform-state --region "ap-northeast-1" --create-bucket-configuration LocationConstraint="ap-northeast-1"
OP_VAULT_NAME="Slack Lab Bot" OP_APP_ENV="Production" op run --env-file provider.env -- aws s3api put-bucket-versioning --bucket slack-lab-bot-terraform-state --versioning-configuration Status=Enabled
OP_VAULT_NAME="Slack Lab Bot" OP_APP_ENV="Production" op run --env-file provider.env -- aws s3api put-bucket-encryption --bucket slack-lab-bot-terraform-state --server-side-encryption-configuration "{\"Rules\" : [{\"ApplyServerSideEncryptionByDefault\" : {\"SSEAlgorithm\" : \"AES256\"}}]}"
OP_VAULT_NAME="Slack Lab Bot" OP_APP_ENV="Production" op run --env-file provider.env -- aws s3api put-public-access-block --bucket slack-lab-bot-terraform-state --public-access-block-configuration "BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true"
```
