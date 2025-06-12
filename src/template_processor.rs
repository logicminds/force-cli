// src/template_processor.rs
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use walkdir::WalkDir;
use serde_json::Value;

use crate::plugin::Plugin;
use crate::runtime_checker::check_required_runtimes;
use crate::renderer::render_template_file;

/// Process and render templates from a directory
pub fn process_templates(
    plugin: &Plugin,
    templates_dir: Option<PathBuf>,
    output_dir: PathBuf,
) -> Result<()> {
    let template_root = match templates_dir {
        Some(dir) => dir,
        None => plugin.templates.clone(),
    };

    let mut template_paths = Vec::new();
    for entry in WalkDir::new(&template_root) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            template_paths.push(path.to_path_buf());
        }
    }

    // Check required runtimes
    check_required_runtimes(&template_paths, Some(plugin))?;

    // TODO: Replace with real variable gathering mechanism
    let variables: Value = serde_json::json!({});

    for input_path in &template_paths {
        let rel_path = input_path.strip_prefix(&template_root)?;
        let output_path = output_dir.join(rel_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        render_template_file(input_path, &output_path, &variables, Some(plugin))?
    }

    Ok(())
}
