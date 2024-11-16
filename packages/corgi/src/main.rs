mod commands;

use std::{
    env,
    fs::create_dir_all,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use dotenvy::dotenv;

use corgi_core::{
    config::{AppConfig, ServerConfig},
    tracing,
};
use corgi_server::CorgiServer;

use commands::Commands;

const DEFAULT_PORT: u16 = 7029;
const DEFAULT_CONFIG_PATH: &str = "./config";
const DEFAULT_DATA_PATH: &str = "./data";

#[derive(Parser)]
#[command(author = "corgi.media")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    /// Specify hostname [default: 127.0.0.1]
    #[arg(long)]
    host: Option<String>,

    /// Specify port [default: 7029]
    #[arg(short, long)]
    port: Option<u16>,

    /// Path to use for configurations [default: ./config]
    #[arg(short, long)]
    config: Option<String>,

    /// Path to use for the data folder (database, caches, etc.)
    /// [default: ./data]
    #[arg(short, long)]
    data: Option<String>,

    /// Database connection URL (protocol://username:password@host/database)
    /// [default: sqlite://{DATA}/database/corgi.db?mode=rwc]
    #[arg(long)]
    database: Option<String>,

    /// Run as a headless service
    #[arg(short, long, default_value = "false")]
    service: bool,

    #[command(subcommand)]
    commands: Option<Commands>,
}

impl Cli {
    fn ip_addr(&self) -> IpAddr {
        let host = self
            .host
            .clone()
            .or(env::var("CORGI_HOST").ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST).to_string());

        IpAddr::from_str(&host).unwrap_or_else(|_| {
            tracing::warn!(
                "Invalid IP address provided: \"{}\". Defaulting to \"{}\"",
                &host,
                Ipv4Addr::LOCALHOST
            );
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        })
    }

    fn port(&self) -> u16 {
        let port = self
            .port
            .or(env::var("CORGI_PORT")
                .ok()
                .and_then(|port| port.parse().ok()))
            .unwrap_or(DEFAULT_PORT);

        match port {
            1024..=49151 => port,
            _ => {
                tracing::warn!(
                    "Port must be between 1024 and 49151. Defaulting to \"{}\"",
                    DEFAULT_PORT
                );
                DEFAULT_PORT
            }
        }
    }

    fn resolve_path(&self, path: &str) -> String {
        let mut path = PathBuf::from(path);
        if self.service {
            let current_exe = std::env::current_exe().unwrap();
            let current_dir = current_exe.parent().unwrap();

            if path.is_relative() {
                path = current_dir.join(&path);
            }
        }

        create_dir_all(&path).unwrap();

        path.to_string_lossy().into_owned()
    }

    fn config_path(&self) -> String {
        let path = self
            .config
            .clone()
            .or(env::var("CORGI_CONFIG_PATH").ok())
            .unwrap_or(DEFAULT_CONFIG_PATH.to_string());

        self.resolve_path(&path)
    }

    fn data_path(&self) -> String {
        let path = self
            .data
            .clone()
            .or(env::var("CORGI_DATA_PATH").ok())
            .unwrap_or(DEFAULT_DATA_PATH.to_string());

        self.resolve_path(&path)
    }

    fn database_url(&self) -> String {
        let url = self
            .database
            .clone()
            .or(env::var("CORGI_DATABASE_URL").ok())
            .unwrap_or_else(|| {
                let default = self.default_database_url();
                tracing::warn!("No database URL provided. Defaulting to {}", default);
                default
            });

        if url.starts_with("sqlite") {
            let path = url.split("://").nth(1).unwrap();
            let path = PathBuf::from(path);

            if let Some(parent) = path.parent() {
                create_dir_all(parent).unwrap();
            }
        }

        url
    }

    fn default_database_url(&self) -> String {
        let path = PathBuf::from(self.data_path()).join("database/corgi.db");

        format!("sqlite://{}?mode=rwc", path.to_string_lossy())
    }
}

impl Cli {
    async fn serve(&self, config: AppConfig) {
        let server = CorgiServer::new(config).await.unwrap();

        server.serve().await.unwrap();
    }
    fn make_config(&self) -> AppConfig {
        let server_config = ServerConfig {
            host: self.ip_addr().to_string(),
            port: self.port(),
            config_path: self.config_path(),
            data_path: self.data_path(),
            database_url: self.database_url(),
        };

        AppConfig::init(server_config)
    }
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    dotenv().ok();
    tracing::init();

    let config = cli.make_config();

    match &cli.commands {
        Some(Commands::Library(library)) => library.run().await,
        None => cli.serve(config).await,
    }
}
