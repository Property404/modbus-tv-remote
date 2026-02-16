use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use mbremote::{CommandServer, key_commander};

#[derive(Parser, Debug)]
struct Args {
    /// The address and port to bind to
    #[clap(default_value = "0.0.0.0:5502")]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let server = CommandServer::new(key_commander);
    println!("Serving on {}", args.addr);
    server.serve(args.addr).await?;

    Ok(())
}
