mod library;

use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use library::{commands::library_commands, Library};

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

#[derive(Subcommand)]
enum Commands {
    /// Library management
    Library(Library),
}

async fn start_server(host: String, port: u16) {
    let address = format!("{}:{}", host, port);
    let addr: SocketAddr = address.parse().unwrap();

    corgi_server::start(addr).await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();

    match &cli.commands {
        Some(Commands::Library(library)) => library_commands(library),
        None => {
            start_server(cli.host, cli.port).await;
        }
    }
}
