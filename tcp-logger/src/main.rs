mod tee_reader;
use clap::{ArgAction, Parser};
use futures::FutureExt;
use std::error::Error;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    bind: Option<String>,

    #[arg(long)]
    server: Option<String>,

    #[clap(long, short, action=ArgAction::SetTrue)]
    without_inbound: bool,

    #[clap(long, short, action=ArgAction::SetTrue)]
    without_outbound: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let listen_addr = args.bind.unwrap_or_else(|| "127.0.0.1:8081".to_string());
    let server_addr = args.server.unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(
            inbound,
            server_addr.clone(),
            args.without_inbound,
            args.without_outbound,
        )
        .map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(
    mut inbound: TcpStream,
    proxy_addr: String,
    without_inbound: bool,
    without_outbound: bool,
) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (ri, mut wi) = inbound.split();
    let (ro, mut wo) = outbound.split();

    let mut ri = tee_reader::TeeReader::new(ri, |buf| {
        if without_inbound || buf.is_empty() {
            return;
        }
        match std::str::from_utf8(buf) {
            Ok(s) => {
                s.split('\n').for_each(|x| println!("> {}", x));
            }
            Err(_) => {
                println!("> {:?}", &buf[..std::cmp::min(256, buf.len())]);
            }
        }
    });

    let mut ro = tee_reader::TeeReader::new(ro, |buf| {
        if without_outbound || buf.is_empty() {
            return;
        }
        match std::str::from_utf8(buf) {
            Ok(s) => {
                s.split('\n').for_each(|x| println!("< {}", x));
            }
            Err(_) => {
                println!("< {:?}", &buf[..std::cmp::min(256, buf.len())]);
            }
        }
    });

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
