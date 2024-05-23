mod commands;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

use corgi_core::{config::AppConfig, tracing};
use corgi_server::CorgiServer;

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

    /// Path to use for configuration
    #[arg(short, long, default_value = "./config")]
    config: String,

    /// Path to use for the data folder (database, caches, etc.)
    #[arg(short, long, default_value = "./data")]
    data: String,

    /// Run as a headless service
    #[arg(short, long, default_value = "false")]
    service: bool,

    #[command(subcommand)]
    commands: Option<Commands>,
}

impl Cli {
    fn ip_addr(&self) -> IpAddr {
        IpAddr::from_str(&self.host).unwrap_or_else(|_| {
            tracing::warn!(
                "Invalid IP address provided: \"{}\". Defaulting to \"{}\"",
                &self.host,
                Ipv4Addr::LOCALHOST
            );
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        })
    }

    fn port(&self) -> u16 {
        self.port.map_or(DEFAULT_PORT, |x| match x {
            1024..=49151 => x,
            _ => {
                tracing::warn!(
                    "Port must be between 1024 and 49151. Defaulting to \"{}\"",
                    DEFAULT_PORT
                );
                DEFAULT_PORT
            }
        })
    }

    fn resolve_service_path(&self, path: &str) -> String {
        let mut path = PathBuf::from(path);
        if self.service {
            let current_exe = std::env::current_exe().unwrap();
            let current_dir = current_exe.parent().unwrap();

            if path.is_relative() {
                path = current_dir.join(&path);
            }
        }
        path.to_string_lossy().into_owned()
    }

    fn config_path(&self) -> String {
        self.resolve_service_path(&self.config)
    }

    fn data_path(&self) -> String {
        self.resolve_service_path(&self.data)
    }
}

impl Cli {
    async fn serve(&self, config: AppConfig) {
        let addr = SocketAddr::new(self.ip_addr(), self.port());

        let server = CorgiServer::new(addr, config).await.unwrap();

        server.serve().await.unwrap();
    }
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = tracing::with_default(|| {
        let config_path = cli.config_path();
        let data_path = cli.data_path();

        tracing::info!("Path to use for configuration: {}", config_path);
        tracing::info!(
            "Path to use for the data folder (database, caches, etc.): {}",
            data_path
        );

        AppConfig::new(&config_path, &data_path)
    });

    tracing::init();

    match &cli.commands {
        Some(Commands::Library(library)) => library.run().await,
        None => cli.serve(config).await,
    }
}
