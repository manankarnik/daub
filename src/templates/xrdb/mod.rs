use crate::themes::Variant;

use anyhow::{anyhow, Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join(".Xresources"),
        format!(
            include_str!(".Xresources"),
            color0 = &variant.color0,
            color1 = &variant.clrd,
            color2 = &variant.clgn,
            color3 = &variant.clyl,
            color4 = &variant.clbl,
            color5 = &variant.clmg,
            color6 = &variant.clcy,
            color7 = &variant.lv05,
            color8 = &variant.lv03,
            color9 = &variant.clrd,
            color10 = &variant.clgn,
            color11 = &variant.clyl,
            color12 = &variant.clbl,
            color13 = &variant.clmg,
            color14 = &variant.clcy,
            color15 = &variant.color15,
            lv00 = &variant.lv00,
            lv05 = &variant.lv05,
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
