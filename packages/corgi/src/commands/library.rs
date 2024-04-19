use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Library {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Parser)]
pub(super) struct ScanArgs {}

#[derive(Subcommand)]
pub(super) enum Commands {
    /// Scan libraries
    Scan(ScanArgs),
}

impl Library {
    pub async fn run(&self) {
        match &self.commands {
            Commands::Scan(_) => {
                println!("Scanning libraries...");
            }
        }
    }
}
