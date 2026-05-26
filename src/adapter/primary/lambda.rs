use crate::adapter::secondary::slack::Client;
use crate::service::activity::{ActivityService, Service};
use chrono::Utc;
use lambda_runtime::{Error, LambdaEvent, service_fn, tracing};
use serde_json::Value;
use std::env;

pub async fn run() -> Result<(), Error> {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(_event: LambdaEvent<Value>) -> Result<(), Error> {
    let token = env::var("SLACK_BOT_TOKEN").map_err(|_| "SLACK_BOT_TOKEN not set")?;
    let weekly_report_channel_id =
        env::var("WEEKLY_REPORT_CHANNEL_ID").map_err(|_| "WEEKLY_REPORT_CHANNEL_ID not set")?;
    let student_ids: Vec<String> = env::var("STUDENT_IDS")
        .map_err(|_| "STUDENT_IDS not set")?
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let oldest_ts = (Utc::now() - chrono::Duration::days(3)).timestamp();
    let service = Service::new(Client::new(token), weekly_report_channel_id);
    let active_students = service.fetch_active_users(oldest_ts).await?;
    let inactive_students: Vec<String> = student_ids
        .into_iter()
        .filter(|id| !active_students.contains(id))
        .collect();
    service.send_reminder(&inactive_students).await?;

    Ok(())
}
