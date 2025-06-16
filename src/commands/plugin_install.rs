// src/commands/plugin_install.rs
use std::fs;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::plugin::{PluginMetadata};

pub fn plugin_install(plugin_name: &str, index_path: PathBuf) -> Result<()> {
    println!("Installing plugin: {}", plugin_name);

    // Simulate plugin installation by creating dummy plugin.json
    let plugin_dir = dirs::home_dir()
        .context("Could not determine home directory")?
        .join(".forge/plugins")
        .join(plugin_name);
    fs::create_dir_all(&plugin_dir)?;

    let plugin_json_path = plugin_dir.join("plugin.json");

    let plugin_metadata = PluginMetadata {
        name: plugin_name.to_string(),
        version: "0.1.0".to_string(),
        description: Some("Sample installed plugin".to_string()),
        templates: String::new(),
        detect: None,
        actions: None,
        custom_renderer_command: None
    };

    let json = serde_json::to_string_pretty(&plugin_metadata)?;
    fs::write(plugin_json_path, json)?;

    // Update plugin-index.json
    let mut index = if index_path.exists() {
        let content = fs::read_to_string(&index_path)?;
        serde_json::from_str::<HashMap<String, serde_json::Value>>(&content)?
    } else {
        HashMap::new()
    };

    let mut entry = serde_json::Map::new();
    entry.insert("installed".to_string(), serde_json::Value::Bool(true));
    entry.insert(
        "path".to_string(),
        serde_json::Value::String(plugin_dir.to_string_lossy().into()),
    );

    index.insert(plugin_name.to_string(), serde_json::Value::Object(entry));

    let json = serde_json::to_string_pretty(&index)?;
    fs::create_dir_all(index_path.parent().unwrap())?;
    fs::write(index_path, json)?;

    println!("Plugin installed: {}", plugin_name);
    Ok(())
}

pub fn plugin_list(installed_plugin_path: PathBuf) -> Result<()> {
    let plugin_list_path = env::current_dir().unwrap_or_default().join("plugin-list.json");
    let mut plugin_index: HashMap<String, serde_json::Value> = HashMap::new();
    let mut installed_plugins: HashMap<String, serde_json::Value> = HashMap::new();

    if ! fs::exists(&installed_plugin_path)? {
        println!("Plugin path {} does not exist.", installed_plugin_path.display());
    } else {
      let installed_plugin_content = fs::read_to_string(installed_plugin_path)?;
      installed_plugins = serde_json::from_str(&installed_plugin_content).unwrap_or_else(|_| HashMap::new());

    }

    if ! fs::exists(&plugin_list_path)?  {
        println!("Plugin index {} not found.", plugin_list_path.display());
    } else {
      let plugin_list_content = fs::read_to_string(plugin_list_path)?;
      plugin_index = serde_json::from_str(&plugin_list_content)
          .context("Failed to parse plugin list content as JSON")?;
    }

    for (name, details) in plugin_index.iter_mut() {
        if let Some(installed_details) = installed_plugins.get(name) {
            details["installed"] = serde_json::Value::Bool(true);
            if let Some(install_path) = installed_details.get("install_path") {
                details["installed_path"] = install_path.clone();
            }
            if let Some(version) = installed_details.get("version") {
                details["installed_version"] = version.clone();
            }
        } else {
            details["installed"] = serde_json::Value::Bool(false);
        }
    }

    println!("Plugins:");
    println!("{:<20} {:<10} {:<10} {}", "Name", "Version", "Installed", "URL");
    println!("{:<20} {:<10} {:<10} {}", "--------------------", "----------", "----------", "--------------------");
    for (name, details) in plugin_index.iter() {
        let version = details.get("version").and_then(|v| v.as_str()).unwrap_or("unknown");
        let installed = details.get("installed")
            .and_then(|v| v.as_bool())
            .map(|b| if b { "✅" } else { "❌" })
            .unwrap_or("❌");
        let git_url = details.get("repo").and_then(|v| v.as_str()).unwrap_or("N/A");
        println!("{:<20} {:<10} {:<10} {}", name, version, installed, git_url);
    }

    Ok(())
}

pub fn plugin_remove(plugin_name: &str, index_path: PathBuf) -> Result<()> {
    if !index_path.exists() {
        anyhow::bail!("Plugin index not found");
    }

    let content = fs::read_to_string(&index_path)?;
    let mut index: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

    if let Some(plugin) = index.get_mut(plugin_name) {
        if let Some(path) = plugin.get("path").and_then(|v| v.as_str()) {
            fs::remove_dir_all(path).ok();
        }
        plugin["installed"] = serde_json::Value::Bool(false);
    }

    let json = serde_json::to_string_pretty(&index)?;
    fs::write(index_path, json)?;

    println!("Plugin removed: {}", plugin_name);
    Ok(())
}
