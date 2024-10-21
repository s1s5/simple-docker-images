use std::collections::HashSet;

use anyhow::Result;
use log::{info, warn};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::config::ConfigHttpPing;

async fn tick(config: &ConfigHttpPing) -> Result<()> {
    let status_set: HashSet<u16> = config
        .status
        .clone()
        .unwrap_or(vec![200])
        .into_iter()
        .collect();
    match reqwest::get(&config.target_url).await {
        Ok(res) => {
            info!("response => {res:?} / config={config:?}");
            if status_set.contains(&res.status().as_u16()) {
                reqwest::get(&config.heartbeat_url).await?;
            }
            Ok(())
        }
        Err(err) => Err(anyhow::anyhow!(
            "Failed to get {}. {err:?}",
            config.target_url
        )),
    }
}

pub async fn add_job(sched: &JobScheduler, config: ConfigHttpPing) -> Result<()> {
    sched
        .add(Job::new_async(config.cron.clone(), move |_uuid, _l| {
            let config = config.clone();
            Box::pin(async move {
                match tick(&config).await {
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
