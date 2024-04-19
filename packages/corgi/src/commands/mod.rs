pub mod library;

use clap::Subcommand;

pub use library::Library;

#[derive(Subcommand)]
pub enum Commands {
    /// Library management
    Library(Library),
}
