use std::path::PathBuf;

use anyhow::Result;

use crate::themes::Variant;

pub mod bspwm;
pub mod kitty;
pub mod nvim;
pub mod rofi;
pub mod shell;
pub mod xrdb;

pub fn generate_all(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    kitty::generate(generated_dir, variant)?;
    nvim::generate(generated_dir, variant)?;
    rofi::generate(generated_dir, variant)?;
    shell::generate(generated_dir, variant)?;
    xrdb::generate(generated_dir, variant)
}

pub fn reload_all(generated_dir: &PathBuf) -> Result<()> {
    bspwm::reload()?;
    kitty::reload(generated_dir)?;
    nvim::reload()?;
    xrdb::reload(generated_dir)
}
