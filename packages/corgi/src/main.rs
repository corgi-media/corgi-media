mod commands;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use clap::Parser;

use commands::Commands;

const DEFAULT_PORT: u16 = 7029;

#[derive(Parser)]
#[command(author = "corgi.media")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    /// Specify hostname
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Specify port [default: 7029]
    #[arg(short, long)]
    port: Option<u16>,

    /// Run as a headless service
    #[arg(short, long, default_value = "false")]
    service: bool,

    #[command(subcommand)]
    commands: Option<Commands>,
}

impl Cli {
    async fn serve(&self) {
        let ip_addr = IpAddr::from_str(&self.host).unwrap_or_else(|_| {
            tracing::warn!(
                "Invalid IP address provided: \"{}\". Defaulting to \"{}\"",
                &self.host,
                Ipv4Addr::LOCALHOST
            );
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        });

        let mut port = self.port.unwrap_or(DEFAULT_PORT);
        if port < 1024 || port > 49151 {
            tracing::warn!(
                "Port must be between 1024 and 49151. Defaulting to \"{}\"",
                DEFAULT_PORT
            );
            port = DEFAULT_PORT;
        }

        let addr = SocketAddr::new(ip_addr, port);

        corgi_server::start(addr).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    corgi_core::init_tracing_subscriber();

    match &cli.commands {
        Some(Commands::Library(library)) => library.run().await,
        None => cli.serve().await,
    }
}
