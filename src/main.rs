mod config;
mod templates;

use anyhow::{Context, Result};
use clap::Parser;
use std::{env, fs::create_dir_all, path::PathBuf};
use toml;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The theme mode to set ("light" or "dark")
    #[arg(short, long)]
    mode: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".config")))
        .context("Failed to find config directory")?
        .join("daub");

    let daub_config = config_dir.join("daub.toml");
    if let Some(parent) = daub_config.parent() {
        create_dir_all(parent)?;
    }

    let default: config::Config =
        toml::from_str(include_str!("config/default.toml")).context("Failed to parse config")?;

    let mut theme = None;
    if args.mode == "dark".to_string() {
        theme = default.dark;
    } else if args.mode == "light".to_string() {
        theme = default.light;
    }

    if let Some(theme) = theme {
        templates::kitty::generate(&config_dir, theme)?;
    }

    Ok(())
}
