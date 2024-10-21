use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use aws_config::Region;
use clap::Parser;
use log::{debug, info};
use tokio::signal::unix::SignalKind;
use tokio_cron_scheduler::JobScheduler;

use super::config::Config;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config_filename: String,
}

pub async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let config: Config = {
        let config_str = std::io::read_to_string(std::fs::File::open(&args.config_filename)?)?;
        let yaml = serde_yaml::yaml_from_str(&config_str)?;
        assert!(yaml.len() == 1);
        serde_yaml::from_yaml(&yaml[0])?
    };

    let sched = JobScheduler::new().await?;

    if let Some(http_list) = config.http {
        for http_config in http_list {
            debug!("http => {http_config:?}");
            super::http::add_job(&sched, http_config).await?;
        }
    }

    if let Some(s3_list) = config.s3 {
        if !s3_list.is_empty() {
            let mut client_map: HashMap<String, Arc<aws_sdk_s3::Client>> = HashMap::new();

            for s3_config in s3_list {
                let client = if let Some(client) = client_map.get(&s3_config.region) {
                    client.clone()
                } else {
                    let region = Region::new(s3_config.region.clone());
                    let sdk_config = aws_config::from_env().region(region).load().await;
                    let client = Arc::new(aws_sdk_s3::Client::new(&sdk_config));
                    client_map.insert(s3_config.region.clone(), client.clone());
                    client
                };

                debug!("s3 => {s3_config:?}");
                super::s3::add_job(&sched, client.clone(), s3_config).await?;
            }
        }
    }

    sched.shutdown_on_signal(SignalKind::terminate());
    sched.start().await?;
    info!("running loop");
    {
        use tokio::signal::{
            ctrl_c,
            unix::{signal, SignalKind},
        };

        let mut sig_int = signal(SignalKind::interrupt()).unwrap();
        let mut sig_term = signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = sig_int.recv() => debug!("SIGINT received"),
            _ = sig_term.recv() => debug!("SIGTERM received"),
            _ = ctrl_c() => debug!("'Ctrl C' received"),
        }
    }

    info!("program terminated");

    Ok(())
}
