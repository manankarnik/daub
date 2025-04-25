use anyhow::{Context, Result};
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    dark: Option<Theme>,
    light: Option<Theme>,
}

#[derive(Deserialize, Debug)]
struct Theme {
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: String,
    color9: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
    background: String,
    foreground: String,
}

fn main() -> Result<()> {
    let default: Config =
        toml::from_str(include_str!("config/default.toml")).context("Failed to parse config")?;
    println!("{default:?}");
    Ok(())
}
