mod adapter;
mod domain;
mod service;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    adapter::primary::lambda::run().await
}
