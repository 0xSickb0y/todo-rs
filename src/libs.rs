//! # todo-rs
//! 
//! A simple and efficient command-line TODO application written in Rust.
//! 
//! ## Overview
//! 
//! todo-rs is a lightweight CLI application for managing daily tasks. It provides
//! a clean, intuitive interface for adding, listing, completing, and removing tasks.
//! All data is stored locally in a SQLite database within your system's config directory.
//! 
//! ## Quick Start
//! 
//! ```bash
//! # Add a task
//! todo-rs add "Buy groceries"
//! 
//! # List all tasks
//! todo-rs list
//! 
//! # Mark task as done
//! todo-rs done 1
//! 
//! # Remove a task
//! todo-rs remove 1
//! ```
//! 
//! ## Architecture
//! 
//! The application is organized into several modules:
//! 
//! - [`args`] - Command line argument parsing
//! - [`config`] - Configuration directory management
//! - [`database`] - Database operations and SQL queries
//! - [`models`] - Data models and database interactions
//! - [`io_utils`] - Input/output utility functions
//! 
//! ## Error Handling
//! 
//! The application uses [`anyhow`] for comprehensive error handling, providing
//! clear error messages and proper error propagation throughout the codebase.
//! 
//! ## Storage
//! 
//! Tasks are stored in a SQLite database located in your system's config directory:
//! - Linux/macOS: `~/.config/todo-rs/tasks.db`
//! - Respects `XDG_CONFIG_HOME` environment variable when set
//! 
//! ## Examples
//! 
//! ### Adding and Managing Tasks
//! 
//! ```bash
//! # Add multiple tasks
//! todo-rs add "Write documentation"
//! todo-rs add "Review pull requests"
//! todo-rs add "Deploy to production"
//! 
//! # List all tasks
//! todo-rs list
//! # Output:
//! # ID       | DONE     | BIRTH               | DESCRIPTION
//! # ------------------------------------------------------------
//! # 1        | false    | 2024-12-07 14:30:15 | Write documentation
//! # 2        | false    | 2024-12-07 14:30:22 | Review pull requests
//! # 3        | false    | 2024-12-07 14:30:25 | Deploy to production
//! 
//! # Complete a task
//! todo-rs done 1
//! 
//! # Remove a task
//! todo-rs remove 3
//! ```

pub mod args;
pub mod config;
pub mod database;
pub mod io_utils;
pub mod models;