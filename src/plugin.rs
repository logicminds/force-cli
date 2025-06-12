// src/plugin.rs
use std::fs;
use std::collections::HashMap;
use std::path::{Path};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub templates: HashMap<String, String>,
    pub detect: Option<PluginDetectRule>,
    pub actions: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginDetectRule {
    pub files: Vec<String>,
}

pub mod plugin_commands {
    use super::*;

    pub fn detect_plugin(path: &Path) -> Option<String> {
        let current_path = env::current_dir().unwrap();
        let index_path = dirs::home_dir()?.join(".forge/plugin-index.json");
        if !index_path.exists() {
            println!("Plugin index not found");
            println!("Using local plugin list");
            println!("Searching for plugins in {}", current_path.display());
            let index_path = current_path.join("plugin-list.json");
            if !index_path.exists() {
                return None;
            }
        }
        let content = fs::read_to_string(index_path).ok()?;
        let list: HashMap<String, serde_json::Value> = serde_json::from_str(&content).ok()?;
        for (name, entry) in list.iter() {
            if let Some(installed) = entry.get("installed") {
                if installed.as_bool() != Some(true) {
                    continue;
                }
            }

            if let Some(files) = entry.get("files") {
                let found_all = files.as_array()?.iter().all(|f| {
                    let file_name = f.as_str().unwrap_or_default();
                    path.join(file_name).exists()
                });

                if found_all {
                    return Some(name.clone());
                }
            }
        }

        None
    }


    pub fn load_plugin_metadata(plugin_name: &str) -> Result<PluginMetadata> {
        let plugin_path = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".forge/plugins")
            .join(plugin_name)
            .join("plugin.json");

        let content = fs::read_to_string(plugin_path)?;
        let metadata: PluginMetadata = serde_json::from_str(&content)?;
        Ok(metadata)
    }
}
