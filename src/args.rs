//! Command line argument parsing module.
//!
//! This module defines the CLI structure and commands using the `clap` crate.
//! It provides a simple interface for parsing command line arguments and
//! returning the appropriate command to execute.

use clap::{Parser, Subcommand};

/// Main CLI structure for the todo-rs application.
///
/// This struct defines the overall command line interface using clap's derive API.
/// It contains a single subcommand field that holds the specific action to perform.
#[derive(Parser, Debug)]
#[command(name = "todo-rs")]
#[command(about = "A simple CLI To-Do app", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the todo-rs application.
///
/// This enum defines all the possible actions that can be performed:
/// - `Add`: Create a new task with a description
/// - `List`: Display all tasks with their status
/// - `Remove`: Delete a task by its ID
/// - `Done`: Mark a task as completed by its ID
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

/// Parse command line arguments and return the command to execute.
///
/// This function uses clap to parse the command line arguments and returns
/// the specific command that should be executed. If parsing fails (due to
/// invalid arguments or help/version requests), clap will handle the output
/// and exit the program automatically.
///
/// # Returns
///
/// Returns the parsed `Commands` enum variant representing the action to perform.
///
/// # Examples
///
/// ```
/// // This would be called automatically from main()
/// let command = parse_args();
/// match command {
///     Commands::Add { description } => { /* handle add */ },
///     Commands::List => { /* handle list */ },
///     // ... other commands
/// }
/// ```
pub fn parse_args() -> Commands {
    let cli = Cli::parse();
    cli.command
}
