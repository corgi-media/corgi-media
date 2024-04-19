mod commands;

use std::net::SocketAddr;

use clap::Parser;

use commands::Commands;

#[derive(Parser)]
#[command(author = "corgi.media")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    /// Specify hostname
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Specify port
    #[arg(short, long, default_value = "7029")]
    port: u16,

    /// Run as a headless service
    #[arg(short, long, default_value = "false")]
    service: bool,

    #[command(subcommand)]
    commands: Option<Commands>,
}

impl Cli {
    async fn serve(&self) {
        let address = format!("{}:{}", self.host, self.port);
        let addr: SocketAddr = address.parse().unwrap();

        corgi_server::start(addr).await;
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();

    match &cli.commands {
        Some(Commands::Library(library)) => library.run().await,
        None => cli.serve().await,
    }
}
