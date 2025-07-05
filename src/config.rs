//! Configuration and directory management module.
//!
//! This module handles all configuration directory operations, including:
//! - Finding the appropriate config directory (XDG compliant)
//! - Creating the application's config directory
//! - Validating directory permissions
//!
//! The module follows the XDG Base Directory specification when available,
//! falling back to the traditional ~/.config directory structure.

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

/// Get the XDG config directory or fall back to ~/.config.
///
/// This function implements the XDG Base Directory specification by first
/// checking the `XDG_CONFIG_HOME` environment variable. If not set, it
/// falls back to `~/.config` as the default config directory.
///
/// # Returns
///
/// Returns a `PathBuf` pointing to the user's config directory.
///
/// # Errors
///
/// Returns an error if the home directory cannot be determined.
///
/// # Examples
///
/// ```
/// let config_dir = get_config_dir()?;
/// println!("Config directory: {}", config_dir.display());
/// ```
pub fn get_config_dir() -> Result<PathBuf> {
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        Ok(PathBuf::from(xdg_config))
    } else {
        let home = get_home_dir()?;
        Ok(PathBuf::from(home).join(".config"))
    }
}

/// Get the application's specific config directory.
///
/// This function returns the path to the todo-rs specific config directory,
/// which is a subdirectory of the user's main config directory.
///
/// # Returns
///
/// Returns a `PathBuf` pointing to `<config_dir>/todo-rs`.
///
/// # Errors
///
/// Returns an error if the config directory cannot be determined.
///
/// # Examples
///
/// ```
/// let app_config = get_app_config_dir()?;
/// // Typically: ~/.config/todo-rs or $XDG_CONFIG_HOME/todo-rs
/// ```
pub fn get_app_config_dir() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("todo-rs"))
}

/// Get the user's home directory.
///
/// This function retrieves the user's home directory from the `HOME`
/// environment variable, which is standard on Unix-like systems.
///
/// # Returns
///
/// Returns the home directory path as a `String`.
///
/// # Errors
///
/// Returns an error if the `HOME` environment variable is not set.
///
/// # Note
///
/// This function is primarily used internally by other config functions.
pub fn get_home_dir() -> Result<String> {
    env::var("HOME").context("Failed to get user home directory. HOME environment variable not set")
}

/// Create the application config directory if it doesn't exist.
///
/// This function ensures that the todo-rs config directory exists and is
/// ready for use. It creates the directory (and any parent directories)
/// if they don't already exist.
///
/// # Returns
///
/// Returns `Ok(())` if the directory exists or was created successfully.
///
/// # Errors
///
/// Returns an error if:
/// - The config directory path cannot be determined
/// - Directory creation fails due to permissions or other filesystem issues
///
/// # Side Effects
///
/// - Creates directory structure if it doesn't exist
/// - Prints a message when creating the directory
///
/// # Examples
///
/// ```
/// // Ensure config directory exists before using it
/// ensure_config_dir()?;
/// ```
pub fn ensure_config_dir() -> Result<()> {
    let config_dir = get_app_config_dir()?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).with_context(|| {
            format!(
                "Failed to create config directory: {}",
                config_dir.display()
            )
        })?;
        println!("Created config directory: {}", config_dir.display());
    }

    Ok(())
}

/// Check if the config directory is writable.
///
/// This function verifies that the application can write to its config
/// directory. This is important for database operations and future
/// configuration file storage.
///
/// # Returns
///
/// Returns `Ok(())` if the directory is writable.
///
/// # Errors
///
/// Returns an error if:
/// - The config directory metadata cannot be read
/// - The directory is marked as read-only
///
/// # Examples
///
/// ```
/// // Check permissions before attempting database operations
/// check_config_dir_writable()?;
/// ```
pub fn check_config_dir_writable() -> Result<()> {
    let config_dir = get_app_config_dir()?;

    let metadata = fs::metadata(&config_dir).with_context(|| {
        format!(
            "Failed to read config directory metadata: {}",
            config_dir.display()
        )
    })?;

    if metadata.permissions().readonly() {
        anyhow::bail!("Config directory is read-only: {}", config_dir.display());
    }

    Ok(())
}
