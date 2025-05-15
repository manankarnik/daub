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
    pub background: String,
    /// Alternate background
    pub background_alt: String,
    /// Selection background
    pub background_selection: String,
    /// Invisible foreground
    pub foreground_invisible: String,
    /// Dark Foreground
    pub foreground_dark: String,
    /// Foreground
    pub foreground: String,
    /// Red
    pub red: String,
    /// Orange
    pub orange: String,
    /// Yellow
    pub yellow: String,
    /// Green
    pub green: String,
    /// Cyan
    pub cyan: String,
    /// Blue
    pub blue: String,
    /// Purple
    pub purple: String,
    /// Brown
    pub brown: String,
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
            if u32::from_str_radix(variant.background.trim_start_matches("#"), 16)
                .context("Failed to parse hex")?
                > u32::from_str_radix(variant.foreground.trim_start_matches("#"), 16)
                    .context("Failed to parse hex")?
            {
                variant.mode = Mode::Light;
                variant.color0 = variant.foreground.clone();
                variant.color7 = variant.background_selection.clone();
                variant.color8 = variant.foreground_dark.clone();
                variant.color15 = variant.background.clone();
            } else {
                variant.mode = Mode::Dark;
                variant.color0 = variant.background.clone();
                variant.color7 = variant.foreground_dark.clone();
                variant.color8 = variant.background_selection.clone();
                variant.color15 = variant.foreground.clone();
            }
        }
    }
    Ok(())
}
