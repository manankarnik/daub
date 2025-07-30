use crate::themes::Variant;

use anyhow::{anyhow, Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("hyprcolors.conf"),
        format!(
            include_str!("hyprcolors.conf"),
            lv00 = &variant.lv00.replace("#", ""),
            lv01 = &variant.lv01.replace("#", ""),
            lv02 = &variant.lv02.replace("#", ""),
            lv03 = &variant.lv03.replace("#", ""),
            lv04 = &variant.lv04.replace("#", ""),
            lv05 = &variant.lv05.replace("#", ""),
            clrd = &variant.clrd.replace("#", ""),
            clor = &variant.clor.replace("#", ""),
            clyl = &variant.clyl.replace("#", ""),
            clgn = &variant.clgn.replace("#", ""),
            clcy = &variant.clcy.replace("#", ""),
            clbl = &variant.clbl.replace("#", ""),
            clmg = &variant.clmg.replace("#", ""),
            clbn = &variant.clbn.replace("#", ""),
        ),
    )
    .context("Failed to write hyprland config file")
}

pub fn reload() -> Result<()> {
    Command::new("hyprctl")
        .arg("reload")
        .output()
        .context("Failed to reload hyprland")?;
    Ok(())
}
