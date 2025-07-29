use crate::themes::Variant;

use anyhow::{anyhow, Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.css"),
        format!(
            include_str!("colors.css"),
            lv00 = &variant.lv00,
            lv01 = &variant.lv01,
            lv02 = &variant.lv02,
            lv03 = &variant.lv03,
            lv04 = &variant.lv04,
            lv05 = &variant.lv05,
            clrd = &variant.clrd,
            clor = &variant.clor,
            clyl = &variant.clyl,
            clgn = &variant.clgn,
            clcy = &variant.clcy,
            clbl = &variant.clbl,
            clmg = &variant.clmg,
            clbn = &variant.clbn,
        ),
    )
    .context("Failed to write waybar config file")
}

pub fn reload() -> Result<()> {
    let waybar_pid = Command::new("pidof")
        .arg("waybar")
        .output()
        .context("Failed to get pid of waybar")?;
    let result = Command::new("kill")
        .args(&[
            "-SIGUSR2",
            String::from_utf8_lossy(&waybar_pid.stdout).trim(),
        ])
        .output()
        .context("Failed to reload waybar")?;
    if !result.status.success() {
        Err(anyhow!("Failed to reload waybar"))?
    }
    Ok(())
}
