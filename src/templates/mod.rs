use std::path::PathBuf;

use anyhow::Result;

use crate::themes::Variant;

pub mod bspwm;
pub mod kitty;
pub mod shell;

pub fn generate_all(config_dir: &PathBuf, variant: &Variant) -> Result<()> {
    kitty::generate(config_dir, variant)?;
    shell::generate(config_dir, variant)
}

pub fn reload_all(config_dir: &PathBuf) -> Result<()> {
    bspwm::reload()?;
    kitty::reload(config_dir)
}
