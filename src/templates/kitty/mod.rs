use crate::themes::Variant;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(config_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        config_dir.join("colors.conf"),
        format!(
            include_str!("colors.conf"),
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
            cursor = &variant.cursor
        ),
    )
    .context("Failed to write kitty config file")?;
    Ok(())
}

pub fn reload(config_dir: &PathBuf) -> Result<()> {
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
                config_dir
                    .join("kitty.conf")
                    .to_str()
                    .context("Failed to get kitty.conf path")?,
            ])
            .output()
            .context("Failed to reload kitty")?;
    }
    Ok(())
}
