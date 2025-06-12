// src/renderer.rs
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, Context};
use tempfile::NamedTempFile;
use std::io::Write;
use serde_json::Value;

use crate::plugin::Plugin;

/// Render a template file using native or external VM-based engines.
pub fn render_template_file(
    input_path: &Path,
    output_path: &Path,
    variables: &Value,
    plugin: Option<&Plugin>,
) -> Result<()> {
    let ext = input_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let script_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts");

    match ext {
        "erb" => render_with_vm("ruby", script_dir.join("render_erb.rb"), input_path, output_path, variables),
        "jinja" => render_with_vm("python3", script_dir.join("render_jinja.py"), input_path, output_path, variables),
        "ejs" => render_with_vm("node", script_dir.join("render_ejs.js"), input_path, output_path, variables),
        "hbs" => render_with_vm("node", script_dir.join("render_hbs.js"), input_path, output_path, variables),
        _ => {
            if let Some(plugin) = plugin {
                if let Some((runtime, command)) = plugin.custom_renderer(ext) {
                    return render_with_plugin_command(runtime, command, input_path, output_path, variables);
                }
            }
            render_fallback(input_path, output_path, variables)
        }
    }
}

/// Fallback simple rendering using Rust string replacement
fn render_fallback(
    input_path: &Path,
    output_path: &Path,
    variables: &Value,
) -> Result<()> {
    let content = fs::read_to_string(input_path)?;
    let mut rendered = content;

    if let Some(obj) = variables.as_object() {
        for (key, value) in obj.iter() {
            let placeholder = format!("{{{{{}}}}}", key);
            rendered = rendered.replace(&placeholder, &value.to_string());
        }
    }

    fs::write(output_path, rendered)?;
    Ok(())
}

/// Render with external VM/runtime using a rendering script and JSON temp file
fn render_with_vm(
    runtime: &str,
    script_path: PathBuf,
    input_path: &Path,
    output_path: &Path,
    variables: &Value,
) -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    let vars_json = serde_json::to_string(variables)?;
    temp_file.write_all(vars_json.as_bytes())?;
    temp_file.flush()?;

    let status = Command::new(runtime)
        .arg(script_path)
        .arg(input_path)
        .arg(output_path)
        .arg(temp_file.path())
        .status()
        .context(format!("Failed to run {} script: {:?}", runtime, script_path))?;

    if !status.success() {
        anyhow::bail!("Rendering failed using {} for: {:?}", runtime, input_path);
    }

    Ok(())
}

/// Fallback renderer using plugin-defined custom command
fn render_with_plugin_command(
    runtime: &str,
    command_path: String,
    input_path: &Path,
    output_path: &Path,
    variables: &Value,
) -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    let vars_json = serde_json::to_string(variables)?;
    temp_file.write_all(vars_json.as_bytes())?;
    temp_file.flush()?;

    let status = Command::new(runtime)
        .arg(command_path)
        .arg(input_path)
        .arg(output_path)
        .arg(temp_file.path())
        .status()
        .context("Failed to run plugin-provided render command")?;

    if !status.success() {
        anyhow::bail!("Rendering via plugin command failed for: {:?}", input_path);
    }

    Ok(())
}
