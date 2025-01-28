use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List all available simulators
    #[arg(short, long)]
    list: bool,

    /// Filter devices by type (e.g., "iphone", "ipad")
    #[arg(name = "filter")]
    filter: Option<String>,
}

fn get_simulators() -> Result<String> {
    let output = Command::new("xcrun")
        .arg("simctl")
        .arg("list")
        .arg("devices")
        .output()
        .context("Failed to execute xcrun simctl command")?;

    String::from_utf8(output.stdout)
        .context("Failed to parse simulator list output")
}

fn should_display_device(device_name: &str, filter: Option<&String>) -> bool {
    match filter {
        Some(f) => device_name.to_lowercase().contains(&f.to_lowercase()),
        None => true,
    }
}

fn display_simulators(output: String, filter: Option<&String>) {
    let mut current_ios_version = String::new();
    
    for line in output.lines() {
        if line.contains("-- iOS") {
            current_ios_version = line.trim()
                .trim_start_matches("--")
                .trim_end_matches("--")
                .trim()
                .to_string();
        } else if line.contains("(") && line.contains(")") {
            let parts: Vec<&str> = line.trim().splitn(2, " (").collect();
            if parts.len() == 2 {
                let device_name = parts[0];
                let device_info = parts[1].trim_end_matches(')');
                
                if should_display_device(device_name, filter) {
                    let status = if device_info.contains("Shutdown") {
                        "Shutdown".red()
                    } else if device_info.contains("Booted") {
                        "Booted".green()
                    } else {
                        "Unknown".yellow()
                    };
                    
                    println!("{} ({}) ({})", 
                        device_name.white(),
                        current_ios_version.cyan(),
                        status);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list {
        let simulators = get_simulators()?;
        display_simulators(simulators, cli.filter.as_ref());
    }

    Ok(())
}
