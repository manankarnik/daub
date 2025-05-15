use crate::themes::Variant;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub fn generate(generated_dir: &PathBuf, variant: &Variant) -> Result<()> {
    fs::write(
        generated_dir.join("colors.sh"),
        format!(
            include_str!("colors.sh"),
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
    .context("Failed to write shell config file")
}
