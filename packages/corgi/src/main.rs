mod library;

use clap::{Parser, Subcommand};

use library::{commands::library_commands, Library};

#[derive(Parser)]
#[command(author = "corgi.media")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    /// Specify hostname
    #[arg(long, default_value = "0.0.0.0")]
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Some(Commands::Library(library)) => library_commands(library),
        None => {
            println!("Hello, world!");
        }
    }
}
