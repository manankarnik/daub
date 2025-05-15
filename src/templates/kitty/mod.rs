use crate::themes::Variant;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.conf"),
        format!(
            include_str!("colors.conf"),
            color0 = &variant.color0,
            color8 = &variant.color8,
            color7 = &variant.color7,
            color15 = &variant.color15,
            lv00 = &variant.lv00,
            lv01 = &variant.lv01,
            lv02 = &variant.lv02,
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
    .context("Failed to write kitty config file")
}

pub fn reload(generated_dir: &PathBuf) -> Result<()> {
    let result = Command::new("pgrep")
        .arg("kitty")
        .output()
        .context("Failed to grep pids of kitty")?;

    if result.status.success() {
        for pid in String::from_utf8(result.stdout)
            .context("Failed to parse stdout")?
            .split("\n")
        {
            Command::new("kill")
                .args(&["-SIGUSR1", pid])
                .output()
                .context("Failed to reload kitty process with pid {pid}")?;
        }
    } else {
        println!(
            "WARN: pgrep failed with {}. {}\nINFO: Using remote control to reload current instance.",
            result.status, String::from_utf8(result.stderr)
                .unwrap_or_default()
                .trim_end()
        );
        Command::new("kitty")
            .args(&[
                "@",
                "set-colors",
                "--all",
                generated_dir
                    .join("kitty.conf")
                    .to_str()
                    .context("Failed to get kitty.conf path")?,
            ])
            .output()
            .context("Failed to reload kitty")?;
    }
    Ok(())
}
