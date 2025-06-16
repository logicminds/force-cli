// src/main.rs
mod plugin;
mod templates;
mod runtime_checker;
mod renderer;
mod template_processor;
mod commands {
    pub mod init;
    pub mod plugin_install;
    pub mod generate;
}

use commands::{
    init::run_forge_init,
    plugin_install::{plugin_install, plugin_list, plugin_remove},
    generate::{GenerateSubcommand, handle_generate},
};
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "AI-powered project scaffolding CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        plugin: Option<String>,

        #[arg(short, long)]
        template: Option<String>,

        #[arg(short, long)]
        force: bool,
    },
    #[command(alias = "gen")]
    Generate {
        #[command(subcommand)]
        action: GenerateSubcommand,
    },
    Plugin {
        #[command(subcommand)]
        action: PluginCommand,
    },
}

#[derive(Subcommand)]
enum PluginCommand {
    Install {
        plugin: String,
    },
    List,
    Remove {
        plugin: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let index_path = dirs::home_dir().unwrap().join(".forge/plugin-index.json");

    match cli.command {
        Commands::Init { plugin, template, force } => {
            run_forge_init(plugin, template, force)?;
        },
        Commands::Generate { action } => {
            handle_generate(action)?;
        },
        Commands::Plugin { action } => match action {
            PluginCommand::Install { plugin } => {
                plugin_install(&plugin, index_path)?;
            },
            PluginCommand::List => {
                plugin_list(index_path)?;
            },
            PluginCommand::Remove { plugin } => {
                plugin_remove(&plugin, index_path)?;
            },
        },
    }
    Ok(())
}
