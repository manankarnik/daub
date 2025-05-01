use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub themes: Vec<Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub variants: HashMap<String, Variant>,
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Variant {
    /// Dark or light mode
    #[serde(skip)]
    pub mode: Mode,
    /// ANSI color0
    #[serde(skip)]
    pub color0: String,
    /// ANSI color7
    #[serde(skip)]
    pub color7: String,
    /// ANSI color8
    #[serde(skip)]
    pub color8: String,
    /// ANSI color15
    #[serde(skip)]
    pub color15: String,
    /// Default background
    pub base00: String,
    /// Lighter background
    pub base01: String,
    /// Selection background
    pub base02: String,
    /// Comments, Invisibles, Line Highlighting
    pub base03: String,
    /// Dark Foreground
    pub base04: String,
    /// Default Foreground
    pub base05: String,
    /// Light Foreground
    pub base06: String,
    /// Lightest Foreground
    pub base07: String,
    /// Red and Bright Red
    pub base08: String,
    /// Orange
    pub base09: String,
    /// Yellow and Bright Yellow
    pub base0A: String,
    /// Green and Bright Green
    pub base0B: String,
    /// Cyan and Bright Cyan
    pub base0C: String,
    /// Blue and Bright Blue
    pub base0D: String,
    /// Purple and Bright Purple
    pub base0E: String,
    /// Dark Red or Brown
    pub base0F: String,
}

pub fn get_preloaded_themes() -> Result<HashMap<String, Theme>> {
    let config: Config = toml::from_str(&[include_str!("example.toml")].join("\n"))
        .context("Predefined themes should be parsable")?;
    let mut themes = HashMap::new();
    for theme in config.themes {
        themes.insert(theme.name.clone(), theme);
    }
    Ok(themes)
}

pub fn define_skipped(themes: &mut HashMap<String, Theme>) -> Result<()> {
    for (_, theme) in themes {
        for (_, variant) in theme.variants.iter_mut() {
            if u32::from_str_radix(variant.base00.trim_start_matches("#"), 16)
                .context("Failed to parse hex")?
                > u32::from_str_radix(variant.base07.trim_start_matches("#"), 16)
                    .context("Failed to parse hex")?
            {
                variant.mode = Mode::Light;
                variant.color0 = variant.base05.clone();
                variant.color7 = variant.base02.clone();
                variant.color8 = variant.base04.clone();
                variant.color15 = variant.base00.clone();
            } else {
                variant.mode = Mode::Dark;
                variant.color0 = variant.base00.clone();
                variant.color7 = variant.base04.clone();
                variant.color8 = variant.base02.clone();
                variant.color15 = variant.base05.clone();
            }
        }
    }
    Ok(())
}
