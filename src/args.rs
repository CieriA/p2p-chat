use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Creates a new room returning a new ticket
    Open {
        #[arg(long)]
        name: Option<String>,
    },
    /// Joins an existing room with a ticket
    Join {
        ticket: String,
        #[arg(long)]
        name: Option<String>,
    },
}
