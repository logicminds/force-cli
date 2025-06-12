// src/templates.rs
use std::process::Command;
use std::path::Path;
use std::fs;
use anyhow::{Result, Context};

pub fn clone_template<P: AsRef<Path>>(url: &str, target: P) -> Result<()> {
    let target = target.as_ref();

    if target.exists() {
        fs::remove_dir_all(target)?;
    }

    let status = Command::new("git")
        .args(["clone", url, target.to_str().unwrap()])
        .status()
        .context("Failed to execute git clone")?;

    if !status.success() {
        anyhow::bail!("Git clone failed for template: {}", url);
    }

    Ok(())
}
