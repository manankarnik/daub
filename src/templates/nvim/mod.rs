use crate::themes::{Mode, Variant};

use anyhow::{Context, Result};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.lua"),
        format!(
            include_str!("colors.lua"),
            mode = match &variant.mode {
                Mode::Dark => "dark",
                Mode::Light => "light",
            },
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
        .context("Failed to find nvim servers")?;

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
                    "execute(\"colorscheme daub\")",
                ])
                .output()
                .context("Failed to reload nvim")?,
        );
    }
    Ok(())
}
