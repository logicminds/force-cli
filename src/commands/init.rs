// src/commands/init.rs
use std::{fs, path::PathBuf};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::plugin::plugin_commands::load_plugin_metadata;
use crate::plugin::plugin_commands::detect_plugin;

use crate::templates::clone_template;

#[derive(Serialize, Deserialize, Debug)]
struct TemplateEntry {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForgeManifest {
    plugin: String,
    templates: Vec<TemplateEntry>,
    created: String,
}

pub fn run_forge_init(plugin_arg: Option<String>, template_arg: Option<String>, force: bool) -> Result<()> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let forge_dir = cwd.join(".forge");
    let manifest_path = forge_dir.join("manifest.json");

    if manifest_path.exists() && !force {
        eprintln!("🛑 Manifest already exists. Use --force to overwrite.");
        std::process::exit(1);
    }

    // Detect plugin
    let plugin = plugin_arg.unwrap_or_else(|| detect_plugin(&cwd).unwrap_or_else(|| {
        eprintln!("✘ Unable to detect plugin. Use --plugin to specify.");
        std::process::exit(1);
    }));

    let plugin_meta = load_plugin_metadata(&plugin).context("Failed to load plugin metadata")?;

    // Determine template URL
    let (template_name, template_url) = match template_arg {
        Some(url) => {
            let name = infer_template_name(&url);
            (name, url)
        },
        None => plugin_meta.templates.iter().next()
            .map(|(k, v)| (k.clone(), v.clone()))
            .expect("Plugin has no templates defined"),
    };

    let target_dir = forge_dir.join("templates").join(&template_name);
    clone_template(&template_url, &target_dir)?;

    // Write manifest
    let manifest = ForgeManifest {
        plugin,
        templates: vec![TemplateEntry { name: template_name, url: template_url.to_string() }],
        created: Utc::now().to_rfc3339(),
    };
    fs::create_dir_all(&forge_dir)?;
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    println!("✅ Initialized .forge/manifest.json with plugin and template.");
    Ok(())
}

fn infer_template_name(url: &str) -> String {
    PathBuf::from(url)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .replace(".git", "")
}
