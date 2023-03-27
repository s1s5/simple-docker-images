use clap::Parser;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use log::{error, info};
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, default_value_t = 8100)]
    port: u16,

    #[arg(long)]
    cache: String,

    #[arg(long)]
    key: String,

    #[arg(long)]
    crt: String,
}

async fn is_shutdown() {
    use async_stream::stream;
    use tokio::signal::unix::{signal, SignalKind};
    use tokio_stream::{StreamExt, StreamMap};

    let signals = vec![SignalKind::interrupt(), SignalKind::terminate()];

    let mut map = StreamMap::new();
    for sig in signals {
        match signal(sig) {
            Ok(signal) => {
                map.insert(
                    sig,
                    Box::pin(stream! {
                        let mut signal = signal;
                        loop {
                            signal.recv().await;
                            yield;
                        }
                    }),
                );
            }
            Err(e) => error!("Failed to enable `{:?}` shutdown signal: {}", sig, e),
        }
    }

    let (_, _) = map.next().await.unwrap();
    // Option<impl futures::stream::Stream<Item = (tokio::signal::unix::SignalKind, ())>>
    // Some(map)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let addr = SocketAddr::from((
        Ipv4Addr::from_str(&args.host).expect("invalid ip v4 addr"),
        args.port,
    ));

    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);

    let proxy_app = Arc::new(http_debugger::app::ProxyApp::new(
        &PathBuf::from(args.cache),
        &args.key,
        &args.crt,
    ));

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        is_shutdown().await;
        match shutdown_tx.send(()) {
            Ok(_) => {}
            Err(_) => error!("Error when shutdown send."),
        }
    });

    tokio::select! {
        _ = async {
            loop {
                let (stream, addr) = match listener.accept().await {
                    Ok(sock) => sock,
                    Err(e) => {
                        error!("Error when accepting {:?}", e);
                        break;
                    }
                };
                info!("stream : {:?}, addr: {:?}", stream, addr);

                let cloned_pa = proxy_app.clone();
                tokio::task::spawn(async move {
                    let cm = cloned_pa;
                    let service = service_fn(move |req| {
                        let cm = cm.clone();
                        http_debugger::proxy::upgradable_proxy(req, cm)
                    });

                    if let Err(err) = http1::Builder::new()
                        .preserve_header_case(true)
                        .title_case_headers(true)
                        .serve_connection(stream, service)
                        .with_upgrades()
                        .await
                    {
                        println!("Failed to serve connection: {:?}", err);
                    }
                });
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        } => {}
        _ = shutdown_rx => {
           info!("shutting down!");
        }
    }

    Ok(())
}
