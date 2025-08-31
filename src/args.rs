use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates a new room returning a new ticket
    Open {
        // TODO use this name
        #[arg(long)]
        name: Option<String>,
    },
    /// Joins an existing room with a ticket
    Join {
        ticket: String,
        // TODO use this name
        #[arg(long)]
        name: Option<String>,
    },
}
