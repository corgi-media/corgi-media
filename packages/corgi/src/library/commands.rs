use super::{Library, LibraryCommands};

pub fn library_commands(library: &Library) {
    match &library.commands {
        LibraryCommands::Scan(_) => {
            println!("Scanning libraries...");
        }
    }
}
