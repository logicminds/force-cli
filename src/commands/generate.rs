// src/commands/generate.rs
// use std::path::{PathBuf};
// use std::fs;
// extern crate fs_extra;
// use anyhow::{Result, Context};
// use crate::plugin::{plugin_commands::load_plugin_metadata};
// use crate::templates::clone_template;

// pub fn generate_file(
//     plugin_name: &str,
//     template_name: &str,
//     target_dir: &PathBuf
// ) -> Result<()> {
//     println!("Generating file...");
//     let plugin = load_plugin_metadata(plugin_name)?;

//     let template_url = plugin.templates.get(template_name)
//         .context("Template not found in plugin metadata")?;

//     let temp_dir = std::env::temp_dir().join("forge-template");
//     if temp_dir.exists() {
//         fs::remove_dir_all(&temp_dir)?;
//     }

//     println!("Cloning template from {}", template_url);
//     clone_template(template_url, &temp_dir)?;

//     // For now, just copy all files from the template
//     fs_extra::dir::copy(
//         &temp_dir,
//         &target_dir,
//         &fs_extra::dir::CopyOptions::new().overwrite(false).copy_inside(true),
//     )?;

//     println!("Template files copied to {:?}", target_dir);

//     // TODO: Add template engine rendering with variables (ejs, erb, jinja, etc.)
//     // TODO: Add interactive preview and editing

//     Ok(())
// }


// src/generate.rs
use clap::Subcommand;
use std::path::PathBuf;
use anyhow::Result;
use crate::plugin::plugin_commands::{detect_plugin};
use crate::plugin::PluginMetadata;
use crate::template_processor::process_templates;
// use std::fs;
// extern crate fs_extra;
// use anyhow::{Result, Context};
// use crate::plugin::{plugin_commands::load_plugin_metadata};
// use crate::templates::clone_template;

#[derive(Subcommand)]
pub enum GenerateSubcommand {
    /// Generate files from templates
    Generate {
        /// Path to output directory
        #[clap(short, long)]
        out: PathBuf,

        /// Optional path to templates (overrides plugin default)
        #[clap(short, long)]
        templates: Option<PathBuf>,
    }
}

pub fn handle_generate(cmd: GenerateSubcommand) -> Result<()> {
    match cmd {
        GenerateSubcommand::Generate { out, templates } => {
            let cwd = std::env::current_dir().unwrap();

            let plugin: PluginMetadata = detect_plugin(&cwd).unwrap();
            process_templates(&plugin, templates, out)?;
        }
    }
    Ok(())
}
