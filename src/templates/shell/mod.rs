use crate::themes::Variant;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.sh"),
        format!(
            include_str!("colors.sh"),
            color0 = &variant.color0,
            color1 = &variant.base08,
            color2 = &variant.base0B,
            color3 = &variant.base0A,
            color4 = &variant.base0D,
            color5 = &variant.base0E,
            color6 = &variant.base0C,
            color7 = &variant.base06,
            color8 = &variant.base03,
            color9 = &variant.base08,
            color10 = &variant.base0B,
            color11 = &variant.base0A,
            color12 = &variant.base0D,
            color13 = &variant.base0E,
            color14 = &variant.base0C,
            color15 = &variant.color15,
            background = &variant.base00,
            foreground = &variant.base07,
            background1 = &variant.base00,
            foreground1 = &variant.base07,
            cursor = &variant.base07
        ),
    )
    .context("Failed to write shell config file")
}
