use crate::themes::{Mode, Variant};

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    let (foreground1, background1) = match variant.mode {
        Mode::Dark => (&variant.color7, &variant.color8),
        Mode::Light => (&variant.color8, &variant.color7),
    };
    fs::write(
        generated_dir.join("colors.sh"),
        format!(
            include_str!("colors.sh"),
            color0 = &variant.color0,
            color1 = &variant.color1,
            color2 = &variant.color2,
            color3 = &variant.color3,
            color4 = &variant.color4,
            color5 = &variant.color5,
            color6 = &variant.color6,
            color7 = &variant.color7,
            color8 = &variant.color8,
            color9 = &variant.color9,
            color10 = &variant.color10,
            color11 = &variant.color11,
            color12 = &variant.color12,
            color13 = &variant.color13,
            color14 = &variant.color14,
            color15 = &variant.color15,
            background = &variant.background,
            foreground = &variant.foreground,
            background1 = background1,
            foreground1 = foreground1,
            cursor = &variant.cursor
        ),
    )
    .context("Failed to write shell config file")
}
