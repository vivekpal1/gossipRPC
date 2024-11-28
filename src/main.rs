mod behaviour;
mod node;
mod rpc;

use clap::{Parser, Subcommand};
use libp2p::Multiaddr;

#[derive(Parser)]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Server {
        #[arg(long, default_value = "/ip4/127.0.0.1/tcp/8080")]
        addr: Multiaddr,
    },
    Call {
        #[arg(long)]
        method: String,
        #[arg(long)]
        params: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Server { addr } => {
            println!("Starting RPC server on {}", addr);
            let mut node = node::Node::new().await?;
            node.start(addr).await?;
        }
        Command::Call { method, params } => {
            println!("Calling method {} with params {}", method, params);
            let params = serde_json::from_str(&params)?;
            let request = rpc::RpcRequest::new(method, params);
            println!("Request created with ID: {}", request.id);
        }
    }

    Ok(())
}