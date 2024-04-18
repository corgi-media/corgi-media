pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Library {
    #[command(subcommand)]
    pub commands: LibraryCommands,
}

#[derive(Parser)]
pub struct LibraryScanArgs {}

#[derive(Subcommand)]
pub enum LibraryCommands {
    /// Scan libraries
    Scan(LibraryScanArgs),
}
