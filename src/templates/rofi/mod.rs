use crate::themes::Variant;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.rasi"),
        format!(
            include_str!("colors.rasi"),
            color0 = &variant.color0,
            color1 = &variant.red,
            color2 = &variant.green,
            color3 = &variant.yellow,
            color4 = &variant.blue,
            color5 = &variant.purple,
            color6 = &variant.cyan,
            color7 = &variant.foreground,
            color8 = &variant.foreground_invisible,
            color9 = &variant.red,
            color10 = &variant.green,
            color11 = &variant.yellow,
            color12 = &variant.blue,
            color13 = &variant.purple,
            color14 = &variant.cyan,
            color15 = &variant.color15,
            background = &variant.background,
            foreground = &variant.foreground,
            background1 = &variant.background,
            foreground1 = &variant.foreground,
        ),
    )
    .context("Failed to write rofi config file")
}
