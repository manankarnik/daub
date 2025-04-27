mod templates;
mod themes;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use std::{env, fs::create_dir_all, path::PathBuf};
use themes::THEMES;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Set a theme
    Set {
        /// Name of the theme
        name: String,
        /// Variant of the theme
        variant: String,
    },
}

fn get_config_dir() -> Result<PathBuf> {
    Ok(env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".config")))
        .context("Failed to find config directory")?
        .join("daub"))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config_dir = get_config_dir()?;
    let daub_config = config_dir.join("daub.toml");
    if let Some(parent) = daub_config.parent() {
        create_dir_all(parent)?;
    }
    let variant = match args.command {
        Command::Set { name, variant } => THEMES
            .get(&name)
            .ok_or(anyhow!("Undefined theme: {name}"))?
            .variants
            .get(&variant)
            .ok_or(anyhow!("Undefined variant: {variant}")),
    }?;

    templates::kitty::generate(&config_dir, variant)?;

    Ok(())
}
