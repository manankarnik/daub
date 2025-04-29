use crate::themes::{Mode, Variant};

use anyhow::{Context, Result};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    let (foreground1, background1) = match variant.mode {
        Mode::Dark => (&variant.color7, &variant.color8),
        Mode::Light => (&variant.color8, &variant.color7),
    };
    fs::write(
        generated_dir.join("colors.vim"),
        format!(
            include_str!("colors.vim"),
            mode = "light",
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
            cursor = &variant.cursor,
        ),
    )
    .context("Failed to write vim config file")
}

pub fn reload(generated_dir: &PathBuf) -> Result<()> {
    let id = String::from_utf8(Command::new("id").arg("-u").output()?.stdout)?;

    // NOTE: We do not check status as expect this command to have success status as false due to the supression of stderr.
    let result = Command::new("find")
        .args(&[
            &format!("/run/user/{}", id.trim()),
            "-type",
            "s",
            "-name",
            "nvim.*",
        ])
        .stderr(Stdio::null())
        .output()
        .context("Failed to grep pids of kitty")?;

    for server in String::from_utf8(result.stdout)
        .context("Failed to parse stdout")?
        .split("\n")
        .filter(|line| !line.is_empty())
    {
        println!("Server: {server}");
        println!(
            "{:?}",
            Command::new("nvim")
                .args(&[
                    "--server",
                    server,
                    "--remote-expr",
                    &format!(
                        "execute(\"source {}\")",
                        generated_dir
                            .join("colors.vim")
                            .to_str()
                            .context("Config path is not valid UTF-8")?,
                    ),
                ])
                .output()
                .context("Failed to reload nvim")?,
        );
    }
    Ok(())
}
