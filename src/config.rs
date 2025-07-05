use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

/// Get the XDG config directory or fall back to ~/.config
pub fn get_config_dir() -> Result<PathBuf> {
    // Check XDG_CONFIG_HOME first (XDG Base Directory spec)
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        Ok(PathBuf::from(xdg_config))
    } else {
        // Fall back to ~/.config
        let home = get_home_dir()?;
        Ok(PathBuf::from(home).join(".config"))
    }
}

/// Get the application's config directory
pub fn get_app_config_dir() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("todo-rs"))
}

/// Get the user's home directory
pub fn get_home_dir() -> Result<String> {
    env::var("HOME")
        .context("Failed to get user home directory. HOME environment variable not set")
}

/// Create the application config directory if it doesn't exist
pub fn ensure_config_dir() -> Result<()> {
    let config_dir = get_app_config_dir()?;
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory: {}", config_dir.display()))?;
        println!("Created config directory: {}", config_dir.display());
    }
    
    Ok(())
}

/// Check if the config directory is writable
pub fn check_config_dir_writable() -> Result<()> {
    let config_dir = get_app_config_dir()?;
    
    let metadata = fs::metadata(&config_dir)
        .with_context(|| format!("Failed to read config directory metadata: {}", config_dir.display()))?;
    
    if metadata.permissions().readonly() {
        anyhow::bail!("Config directory is read-only: {}", config_dir.display());
    }
    
    Ok(())
}