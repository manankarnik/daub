use crate::themes::Variant;

use anyhow::{anyhow, Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join(".Xresources"),
        format!(
            include_str!(".Xresources"),
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
        ),
    )
    .context("Failed to write .Xresources config file")
}

pub fn reload(generated_dir: &PathBuf) -> Result<()> {
    let result = Command::new("xrdb")
        .args(&[
            "-merge",
            generated_dir
                .join(".Xresources")
                .to_str()
                .context("Failed to get .Xresources path")?,
        ])
        .output()
        .context("Failed to reload bspwm")?;
    if !result.status.success() {
        Err(anyhow!("Failed to reload bspwm"))?
    }
    Ok(())
}
