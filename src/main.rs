use anyhow::{Context, Result, anyhow};
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

    /// Boot a specific simulator by name (e.g., "iPhone 15 Pro")
    #[arg(long)]
    boot: Option<String>,
}

#[derive(Debug)]
struct Device {
    name: String,
    udid: String,
    ios_version: String,
    status: String,
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

fn parse_devices(output: String) -> Vec<Device> {
    let mut devices = Vec::new();
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
                let name = parts[0].to_string();
                let info_parts: Vec<&str> = parts[1].split(") (").collect();
                if info_parts.len() == 2 {
                    let udid = info_parts[0].to_string();
                    let status = info_parts[1].trim_end_matches(')').to_string();
                    devices.push(Device {
                        name,
                        udid,
                        ios_version: current_ios_version.clone(),
                        status,
                    });
                }
            }
        }
    }
    devices
}

fn display_simulators(devices: &[Device], filter: Option<&String>) {
    for device in devices {
        if should_display_device(&device.name, filter) {
            let status = if device.status.contains("Shutdown") {
                "Shutdown".red()
            } else if device.status.contains("Booted") {
                "Booted".green()
            } else {
                "Unknown".yellow()
            };
            
            println!("{} ({}) ({})", 
                device.name.white(),
                device.ios_version.cyan(),
                status);
        }
    }
}

fn boot_simulator(device_name: &str, devices: &[Device]) -> Result<()> {
    let device = devices.iter()
        .find(|d| d.name.to_lowercase() == device_name.to_lowercase())
        .ok_or_else(|| anyhow!("Device '{}' not found", device_name))?;

    if device.status.contains("Booted") {
        println!("{} {} {}", 
            device.name.white(),
            "is already booted".yellow(),
            "✓".green());
        return Ok(());
    }

    println!("{} {} {}", 
        "Booting".cyan(),
        device.name.white(),
        "...".cyan());

    let output = Command::new("xcrun")
        .arg("simctl")
        .arg("boot")
        .arg(&device.udid)
        .output()
        .context("Failed to boot simulator")?;

    if output.status.success() {
        // Open the Simulator.app after booting
        Command::new("open")
            .arg("-a")
            .arg("Simulator")
            .output()
            .context("Failed to open Simulator.app")?;

        println!("{} {} {}", 
            device.name.white(),
            "booted successfully".green(),
            "✓".green());
        Ok(())
    } else {
        Err(anyhow!("Failed to boot simulator: {}", 
            String::from_utf8_lossy(&output.stderr)))
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let simulators = get_simulators()?;
    let devices = parse_devices(simulators);

    if cli.list {
        display_simulators(&devices, cli.filter.as_ref());
    } else if let Some(device_name) = cli.boot {
        boot_simulator(&device_name, &devices)?;
    }

    Ok(())
}
