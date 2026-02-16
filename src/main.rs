use anyhow::Result;
use mbremote::{CommandServer, key_commander};

#[tokio::main]
async fn main() -> Result<()> {
    let server = CommandServer::new(key_commander);
    server.serve("0.0.0.0:5502".parse()?).await?;

    Ok(())
}
