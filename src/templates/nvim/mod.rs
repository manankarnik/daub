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
            mode = match &variant.mode {
                Mode::Dark => "dark",
                Mode::Light => "light",
            },
            base00 = &variant.base00,
            base01 = &variant.base01,
            base02 = &variant.base02,
            base03 = &variant.base03,
            base04 = &variant.base04,
            base05 = &variant.base05,
            base06 = &variant.base06,
            base07 = &variant.base07,
            base08 = &variant.base08,
            base09 = &variant.base09,
            base0A = &variant.base0A,
            base0B = &variant.base0B,
            base0C = &variant.base0C,
            base0D = &variant.base0D,
            base0E = &variant.base0E,
            base0F = &variant.base0F,
        ),
    )
    .context("Failed to write vim config file")
}

pub fn reload() -> Result<()> {
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
                    "execute(\"source $MYVIMRC\")",
                ])
                .output()
                .context("Failed to reload nvim")?,
        );
    }
    Ok(())
}
