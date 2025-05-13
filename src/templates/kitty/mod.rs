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
            base00 = &variant.base00,
            base01 = &variant.base01,
            base02 = &variant.base02,
            // base03 = &variant.base03,
            base04 = &variant.base04,
            base05 = &variant.base05,
            base06 = &variant.base06,
            base08 = &variant.base08,
            base09 = &variant.base09,
            base0A = &variant.base0A,
            base0B = &variant.base0B,
            base0C = &variant.base0C,
            base0D = &variant.base0D,
            base0E = &variant.base0E,
            base0F = &variant.base0F,
            cursor = &variant.ui.cursor
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
