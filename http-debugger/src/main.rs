use clap::Parser;
use hyper::server::conn::http1;
use hyper::service::service_fn;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = SocketAddr::from((
        Ipv4Addr::from_str(&args.host).expect("invalid ip v4 addr"),
        args.port,
    ));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let proxy_app = Arc::new(http_debugger::app::ProxyApp::new(
        &PathBuf::from(args.cache),
        &args.key,
        &args.crt,
    ));

    loop {
        let (stream, _) = listener.accept().await?;
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
}
