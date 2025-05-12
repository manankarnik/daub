mod templates;
mod themes;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use std::{collections::HashMap, env, fs, path::PathBuf};
use themes::{define_skipped, get_preloaded_themes, Config, Theme};

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
    /// Clean generated files
    Clean,

    /// List themes and variants
    List,
}

fn get_config_dir() -> Result<PathBuf> {
    Ok(env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".config")))
        .context("Failed to find config directory")?
        .join("daub"))
}

fn parse_config(daub_config: &PathBuf) -> Result<HashMap<String, Theme>> {
    let mut themes = get_preloaded_themes()?;
    if daub_config.exists() {
        let config: Config = toml::from_str(
            &String::from_utf8(fs::read(daub_config).context("Failed to read config from disk")?)
                .context("Config is not valid UTF-8")?,
        )
        .context("Failed to parse config from TOML")?;
        for theme in config.themes {
            if themes.get(&theme.name).is_some() {
                Err(anyhow!(format!("Theme {} already exists", theme.name)))?
            }
            themes.insert(theme.name.clone(), theme);
        }
    }
    define_skipped(&mut themes)?;
    Ok(themes)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config_dir = get_config_dir()?;
    let daub_config = config_dir.join("daub.toml");
    let themes = parse_config(&daub_config)?;
    let generated_dir = config_dir.join("gen");
    if !generated_dir.exists() {
        fs::create_dir_all(&generated_dir)?
    }

    match args.command {
        Command::Set { name, variant } => {
            let variant = themes
                .get(&name)
                .ok_or(anyhow!("Undefined theme: {name}"))?
                .variants
                .get(&variant)
                .ok_or(anyhow!("Undefined variant: {variant}"))?;
            templates::generate_all(&generated_dir, variant)?;
            templates::reload_all(&generated_dir)?;
        }
        Command::Clean => {
            fs::remove_dir_all(&generated_dir).context("Failed to remove generated files")?;
        }
        Command::List => {
            println!("Available Themes:\n");
            let mut themes: Vec<_> = themes.iter().collect();
            themes.sort_by_key(|&(n, _)| n);
            for (name, theme) in themes {
                let mut variants = theme
                    .variants
                    .keys()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>();
                variants.sort();
                println!("  • {name}");
                if let Some(author) = &theme.author {
                    println!("     Author: {}", author);
                }
                println!("     Variants: [{}]\n", variants.join(", "));
            }
        }
    }
    Ok(())
}
