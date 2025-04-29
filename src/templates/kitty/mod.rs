use crate::themes::Variant;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.conf"),
        format!(
            include_str!("colors.conf"),
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
            selection_background = &variant.base02,
            selection_foreground = &variant.base06,
            active_tab_background = &variant.base0E,
            active_tab_foreground = &variant.base04,
            inactive_tab_background = &variant.base01,
            inactive_tab_foreground = &variant.base05,
            active_border_color = &variant.base0C,
            inactive_border_color = &variant.base01,
            cursor = &variant.base07,
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
