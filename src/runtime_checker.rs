// src/runtime_checker.rs
use std::collections::{HashSet};
use std::process::Command;
use std::path::Path;
use anyhow::{Result, Context};

use crate::plugin::PluginMetadata;

/// Built-in mappings of file extensions to runtimes
pub fn builtin_runtime_for_extension(ext: &str) -> Option<&'static str> {
    match ext {
        "erb" => Some("ruby"),
        "jinja" => Some("python3"),
        "ejs" | "hbs" => Some("node"),
        _ => None,
    }
}

/// Check required runtimes based on extensions and plugin overrides
pub fn check_required_runtimes<P: AsRef<Path>>(
    template_paths: &[P],
    plugin: Option<&PluginMetadata>,
) -> Result<()> {
    let mut required: HashSet<String> = HashSet::new();

    for path in template_paths {
        if let Some(ext) = path.as_ref().extension().and_then(|e| e.to_str()) {
            // Check plugin overrides first
            if let Some(plugin) = plugin {
              if let Some(runtime) = plugin.custom_renderer_command {
                  required.insert(runtime.to_string());
                  continue;
              }
            }
            // Fallback to builtin runtime
            if let Some(runtime) = builtin_runtime_for_extension(ext) {
                required.insert(runtime.to_string());
            }
        }
    }

    for runtime in required {
        let status = Command::new(&runtime)
            .arg("--version")
            .output()
            .with_context(|| format!("Failed to check runtime '{}'. Is it installed?", runtime))?;

        if !status.status.success() {
            anyhow::bail!("Required runtime '{}' is not available.", runtime);
        }
    }

    Ok(())
}
