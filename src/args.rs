//! A module for parsing the command line arguments for nexus.
pub use clap::Parser;
use clap::{command, Subcommand};

/// Command line arguments for nexus.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new note.
    #[command(arg_required_else_help = true)]
    Create {
        /// The name of the note to create.
        name: String,
    },
}
