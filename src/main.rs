use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use mbremote::{CommandServer, key_commander, run_client};

#[derive(Parser, Debug)]
struct Server {
    /// The address and port to bind to
    #[clap(default_value = "0.0.0.0:5502")]
    addr: SocketAddr,
}

#[derive(Parser, Debug)]
enum Args {
    Server {
        /// The address and port to bind to
        #[clap(default_value = "0.0.0.0:5502")]
        addr: SocketAddr,
    },
    Client {
        /// The address and port to attach to
        addr: SocketAddr,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args {
        Args::Server { addr } => {
            let server = CommandServer::new(key_commander);
            println!("Serving on {}", addr);
            server.serve(addr).await?;
        }
        Args::Client { addr } => {
            run_client(addr).await?;
        }
    }

    Ok(())
}
