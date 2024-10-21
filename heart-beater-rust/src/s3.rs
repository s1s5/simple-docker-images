use std::sync::Arc;

use anyhow::Result;
use aws_sdk_s3::types::Object;
use log::{debug, info, warn};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::config::ConfigS3Ping;

async fn get_latest_object(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    prefix: &str,
) -> Option<Object> {
    let mut objects = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix(prefix)
        .into_paginator()
        .send();

    let mut latest_object: Option<Object> = None;
    while let Some(Ok(object_lis)) = objects.next().await {
        for object in object_lis.contents() {
            debug!("get object => {object:?}");
            if let Some(last_modified) = object.last_modified() {
                if let Some(lo) = latest_object.as_ref() {
                    if let Some(dt) = lo.last_modified() {
                        if last_modified > dt {
                            latest_object = Some(object.clone());
                        }
                    } else {
                        latest_object = Some(object.clone());
                    }
                } else {
                    latest_object = Some(object.clone());
                }
            }
        }
    }

    latest_object
}

async fn tick(client: &aws_sdk_s3::Client, config: &ConfigS3Ping) -> Result<()> {
    match get_latest_object(client, &config.bucket, &config.prefix).await {
        Some(object) => {
            info!("latest object => {object:?} / config={config:?}");
            if let Some(min_size) = config.min_size.as_ref() {
                let size = object
                    .size()
                    .ok_or(anyhow::anyhow!("key={:?} no size found", object.key()))?;
                if size < (*min_size as i64) {
                    return Ok(());
                }
            }
            let at = object
                .last_modified()
                .ok_or(anyhow::anyhow!(
                    "key={:?} no last_modified found",
                    object.key()
                ))?
                .to_millis()?;

            if at + config.grace.as_millis() as i64 > chrono::Utc::now().timestamp_millis() {
                reqwest::get(&config.heartbeat_url).await?;
            }

            Ok(())
        }
        None => Err(anyhow::anyhow!("not data found. or failed to access")),
    }
}

pub async fn add_job(
    sched: &JobScheduler,
    client: Arc<aws_sdk_s3::Client>,
    config: ConfigS3Ping,
) -> Result<()> {
    sched
        .add(Job::new_async(config.cron.clone(), move |_uuid, _l| {
            let client = client.clone();
            let config = config.clone();
            Box::pin(async move {
                match tick(&client, &config).await {
                    Ok(_) => {}
                    Err(err) => {
                        warn!("failed to access {config:?} {err:?}");
                    }
                }
            })
        })?)
        .await?;
    Ok(())
}
