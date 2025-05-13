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
            background = &variant.background,
            background_alt = &variant.background_alt,
            background_selection = &variant.background_selection,
            foreground_invisible = &variant.foreground_invisible,
            foreground_dark = &variant.foreground_dark,
            foreground = &variant.foreground,
            red = &variant.red,
            orange = &variant.orange,
            yellow = &variant.yellow,
            green = &variant.green,
            cyan = &variant.cyan,
            blue = &variant.blue,
            purple = &variant.purple,
            brown = &variant.brown,
            string = &variant.syntax.string,
            function = &variant.syntax.function,
            r#macro = &variant.syntax.r#macro,
            builtin = &variant.syntax.builtin,
            keyword = &variant.syntax.keyword,
            comment = &variant.syntax.comment,
            r#type = &variant.syntax.r#type,
            constant = &variant.syntax.constant,
            identifier = &variant.syntax.identifier,
            cursor = &variant.ui.cursor,
            cursor_line = &variant.ui.cursor_line,
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
