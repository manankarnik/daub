use std::process::Command;

use anyhow::{anyhow, Context, Result};

pub fn reload() -> Result<()> {
    let result = Command::new("bspc")
        .args(&["wm", "-r"])
        .output()
        .context("Failed to reload bspwm")?;
    if !result.status.success() {
        Err(anyhow!("Failed to reload bspwm"))?
    }
    Ok(())
}
