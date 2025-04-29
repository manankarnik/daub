use crate::themes::{Mode, Variant};

use anyhow::{Context, Result};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.vim"),
        format!(
            include_str!("colors.vim"),
            mode = "light",
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
            cursor = &variant.base07,
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
