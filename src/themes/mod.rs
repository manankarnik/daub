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
    pub author: String,
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
    #[serde(skip)]
    pub mode: Mode,
    #[serde(skip)]
    pub color0: String,
    #[serde(skip)]
    pub color15: String,
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    pub base0A: String,
    pub base0B: String,
    pub base0C: String,
    pub base0D: String,
    pub base0E: String,
    pub base0F: String,
}

pub fn get_preloaded_themes() -> Result<HashMap<String, Theme>> {
    let config: Config =
        toml::from_str(&[include_str!("3024.toml"), include_str!("default.toml")].join("\n"))
            .context("Predefined themes should be parsable")?;
    let mut themes = HashMap::new();
    for mut theme in config.themes {
        for (_, variant) in theme.variants.iter_mut() {
            if u32::from_str_radix(variant.base00.trim_start_matches("#"), 16)
                .context("Failed to parse hex")?
                > u32::from_str_radix(variant.base07.trim_start_matches("#"), 16)
                    .context("Failed to parse hex")?
            {
                variant.mode = Mode::Light;
                variant.color0 = variant.base07.clone();
                variant.color15 = variant.base00.clone();
            } else {
                variant.mode = Mode::Dark;
                variant.color0 = variant.base00.clone();
                variant.color15 = variant.base07.clone();
            }
        }
        themes.insert(theme.name.clone(), theme);
    }
    Ok(themes)
}
