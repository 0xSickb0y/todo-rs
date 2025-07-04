use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo-rs")]
#[command(about = "A simple CLI To-Do app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Add a new task")]
    Add { description: String },
    #[command(about = "List all tasks")]
    List,
    #[command(about = "Remove a task by ID")]
    Remove { id: i64 },
    #[command(about = "Mark a task as 'done' by ID")]
    Done { id: i64 },
}

pub fn parse_args() -> Commands {
    let cli = Cli::parse();
    cli.command
}
