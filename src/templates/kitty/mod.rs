use crate::config::Theme;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf, process::Command};

pub fn generate(config_dir: &PathBuf, theme: Theme) -> Result<()> {
    fs::write(
        config_dir.join("kitty.conf"),
        format!(
            include_str!("kitty.conf"),
            color0 = theme.color0,
            color1 = theme.color1,
            color2 = theme.color2,
            color3 = theme.color3,
            color4 = theme.color4,
            color5 = theme.color5,
            color6 = theme.color6,
            color7 = theme.color7,
            color8 = theme.color8,
            color9 = theme.color9,
            color10 = theme.color10,
            color11 = theme.color11,
            color12 = theme.color12,
            color13 = theme.color13,
            color14 = theme.color14,
            color15 = theme.color15,
            background = theme.background,
            foreground = theme.foreground
        ),
    )
    .context("Failed to write config file")?;

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
