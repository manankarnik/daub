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
    /// Level 0: Background
    pub lv00: String,
    /// level 1: Alternate Background
    pub lv01: String,
    /// level 2: Selection Background
    pub lv02: String,
    /// level 3: Muted foreground
    pub lv03: String,
    /// level 4: Alternate Foreground
    pub lv04: String,
    /// level 5: Foreground
    pub lv05: String,
    /// Color Red
    pub clrd: String,
    /// Color Orange
    pub clor: String,
    /// Color Yellow
    pub clyl: String,
    /// Color Green
    pub clgn: String,
    /// Color Cyan
    pub clcy: String,
    /// Color Blue
    pub clbl: String,
    /// Color Magenta
    pub clmg: String,
    /// Color Brown
    pub clbn: String,
}

pub fn get_preloaded_themes() -> Result<HashMap<String, Theme>> {
    let config: Config = toml::from_str(&[include_str!("example.toml")].join("\n"))
        .context("Pre-defined themes should be parsable")?;
    let mut themes = HashMap::new();
    for theme in config.themes {
        themes.insert(theme.name.clone(), theme);
    }
    Ok(themes)
}

pub fn define_skipped(themes: &mut HashMap<String, Theme>) -> Result<()> {
    for (_, theme) in themes {
        for (_, variant) in theme.variants.iter_mut() {
            if u32::from_str_radix(variant.lv00.trim_start_matches("#"), 16)
                .context("Failed to parse hex")?
                > u32::from_str_radix(variant.lv05.trim_start_matches("#"), 16)
                    .context("Failed to parse hex")?
            {
                variant.mode = Mode::Light;
                variant.color0 = variant.lv05.clone();
                variant.color7 = variant.lv01.clone();
                variant.color8 = variant.lv04.clone();
                variant.color15 = variant.lv00.clone();
            } else {
                variant.mode = Mode::Dark;
                variant.color0 = variant.lv00.clone();
                variant.color7 = variant.lv04.clone();
                variant.color8 = variant.lv01.clone();
                variant.color15 = variant.lv05.clone();
            }
        }
    }
    Ok(())
}
