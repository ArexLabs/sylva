use anyhow::Result;
use clap::Parser;
use gio::prelude::*;
use gio::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Sylva Declarative Configuration Engine")]
struct Args {
    /// Path to declarative YAML configuration
    #[arg(short, long, default_value = "/etc/sylva/config.yaml")]
    config: PathBuf,

    /// Dry run mode
    #[arg(short, long)]
    dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct SylvaConfig {
    desktop: DesktopConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct DesktopConfig {
    theme: String,
    font: String,
    panel_height: i32,
    effects_enabled: bool,
}

fn apply_config(config: SylvaConfig, dry_run: bool) -> Result<()> {
    println!("Applying Sylva configuration...");

    let interface_settings = Settings::new("org.cinnamon.desktop.interface");
    let wm_settings = Settings::new("org.cinnamon.desktop.wm.preferences");

    if !dry_run {
        interface_settings.set_string("gtk-theme", &config.desktop.theme)?;
        interface_settings.set_string("font-name", &config.desktop.font)?;
        println!("Set GTK theme to: {}", config.desktop.theme);
        println!("Set Default font to: {}", config.desktop.font);
    } else {
        println!("[DRY RUN] Would set GTK theme to: {}", config.desktop.theme);
        println!(
            "[DRY RUN] Would set Default font to: {}",
            config.desktop.font
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.config.exists() {
        println!("Config file not found: {:?}", args.config);
        return Ok(());
    }

    let content = fs::read_to_string(args.config)?;
    let config: SylvaConfig = serde_yaml::from_str(&content)?;

    apply_config(config, args.dry_run)?;

    Ok(())
}
