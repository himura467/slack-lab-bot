# Bash commands

- `./scripts/deploy.sh prod`: Deploy to production
- `./scripts/build.sh`: Build Lambda binary

# Scripts overview

## Deployment workflow

1. Build the Rust Lambda binary with `./scripts/build.sh`
2. Deploy infrastructure with `./scripts/deploy.sh prod`
3. Requires 1Password CLI with "Slack Lab Bot" vault access and `cargo-lambda` installed

## Script details

### `deploy.sh`

- Takes environment parameter (currently only `prod` supported)
- Runs in `/terraform/environments/{env}/` directory
- Uses 1Password CLI integration: `OP_VAULT_NAME="Slack Lab Bot" OP_APP_ENV="Production"`
- Executes `terraform init` and `terraform apply`

### `build.sh`

- Compiles the Rust Lambda binary for `x86_64-unknown-linux-gnu` via `cargo lambda build`
- Packages the binary as `slack-lab-bot.zip` in the project root
- Must be run before Terraform deployment
