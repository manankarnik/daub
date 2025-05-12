use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

macro_rules! set_syntax_field {
    ($theme:expr, $variant:expr, $field:ident, $color:ident) => {
        println!("{}", stringify!($field));
        $variant.syntax.$field = $variant.$color.clone();
        if let Some(syntax) = &$theme.syntax {
            let field = stringify!($field);
            if let Some(value) = syntax.get(field.strip_prefix("r#").unwrap_or(field)) {
                $variant.syntax.$field = String::from(match value as &str {
                    "base00" => Ok(&$variant.base00),
                    "base01" => Ok(&$variant.base01),
                    "base02" => Ok(&$variant.base02),
                    "base03" => Ok(&$variant.base03),
                    "base04" => Ok(&$variant.base04),
                    "base05" => Ok(&$variant.base05),
                    "base06" => Ok(&$variant.base06),
                    "base07" => Ok(&$variant.base07),
                    "base08" => Ok(&$variant.base08),
                    "base09" => Ok(&$variant.base09),
                    "base0A" => Ok(&$variant.base0A),
                    "base0B" => Ok(&$variant.base0B),
                    "base0C" => Ok(&$variant.base0C),
                    "base0D" => Ok(&$variant.base0D),
                    "base0E" => Ok(&$variant.base0E),
                    "base0F" => Ok(&$variant.base0F),
                    _ => Err(anyhow!("Undefined color")),
                }?);
            }
        }
    };
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub themes: Vec<Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub syntax: Option<HashMap<String, String>>,
    pub variants: HashMap<String, Variant>,
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Default)]
pub struct Syntax {
    pub string: String,
    pub function: String,
    pub builtin: String,
    pub keyword: String,
    pub comment: String,
    pub r#type: String,
    pub constant: String,
    pub identifier: String,
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

    /// Syntax overrides
    #[serde(skip)]
    pub syntax: Syntax,
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
            set_syntax_field!(theme, variant, string, base0B);
            set_syntax_field!(theme, variant, function, base0D);
            set_syntax_field!(theme, variant, builtin, base0A);
            set_syntax_field!(theme, variant, keyword, base0E);
            set_syntax_field!(theme, variant, comment, base03);
            set_syntax_field!(theme, variant, r#type, base0A);
            set_syntax_field!(theme, variant, constant, base09);
            set_syntax_field!(theme, variant, identifier, base06);
        }
    }
    Ok(())
}
