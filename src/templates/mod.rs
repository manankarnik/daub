use std::path::PathBuf;

use anyhow::Result;

use crate::themes::Variant;

pub mod bspwm;
pub mod hypr;
pub mod kitty;
pub mod nvim;
pub mod rofi;
pub mod shell;
pub mod waybar;
pub mod xrdb;

pub fn generate_all(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    hypr::generate(generated_dir, variant)?;
    kitty::generate(generated_dir, variant)?;
    nvim::generate(generated_dir, variant)?;
    rofi::generate(generated_dir, variant)?;
    shell::generate(generated_dir, variant)?;
    waybar::generate(generated_dir, variant)
    // xrdb::generate(generated_dir, variant)
}

pub fn reload_all(generated_dir: &PathBuf) -> Result<()> {
    // bspwm::reload()?;
    hypr::reload()?;
    kitty::reload(generated_dir)?;
    nvim::reload()?;
    waybar::reload()
    // xrdb::reload(generated_dir)
}
