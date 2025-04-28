use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Themes {
    pub themes: Vec<Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub variants: HashMap<String, Variant>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Mode {
    #[serde(alias = "dark")]
    Dark,
    #[serde(alias = "light")]
    Light,
}

#[derive(Deserialize, Debug)]
struct PartialVariants {
    mode: Mode,
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: Option<String>,
    color9: Option<String>,
    color10: Option<String>,
    color11: Option<String>,
    color12: Option<String>,
    color13: Option<String>,
    color14: Option<String>,
    color15: Option<String>,
    background: String,
    foreground: String,
    cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub mode: Mode,
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let p = PartialVariants::deserialize(deserializer)?;
        Ok(Variant {
            mode: p.mode,
            color8: p.color8.unwrap_or_else(|| p.color0.clone()),
            color9: p.color9.unwrap_or_else(|| p.color1.clone()),
            color10: p.color10.unwrap_or_else(|| p.color2.clone()),
            color11: p.color11.unwrap_or_else(|| p.color3.clone()),
            color12: p.color12.unwrap_or_else(|| p.color4.clone()),
            color13: p.color13.unwrap_or_else(|| p.color5.clone()),
            color14: p.color14.unwrap_or_else(|| p.color6.clone()),
            color15: p.color15.unwrap_or_else(|| p.color7.clone()),
            color0: p.color0,
            color1: p.color1,
            color2: p.color2,
            color3: p.color3,
            color4: p.color4,
            color5: p.color5,
            color6: p.color6,
            color7: p.color7,
            background: p.background,
            cursor: p.cursor.unwrap_or_else(|| p.foreground.clone()),
            foreground: p.foreground,
        })
    }
}

pub fn get_preloaded_themes() -> Result<HashMap<String, Theme>> {
    let parsed: Themes = toml::from_str(
        &[
            // include_str!("../themes/3024.toml"),
            include_str!("../themes/default.toml"),
        ]
        .join("\n"),
    )
    .context("Predefined themes should be parsable")?;
    let mut themes = HashMap::new();
    for theme in parsed.themes {
        themes.insert(theme.name.clone(), theme);
    }
    Ok(themes)
}
