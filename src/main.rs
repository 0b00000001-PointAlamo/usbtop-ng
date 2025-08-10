use anyhow::Result;
use clap::Parser;
use log::{info, error, warn};
use std::process;
use std::path::Path;
use std::env;
use std::io::{self, Write};
use std::fs::OpenOptions;

mod usbmon;
mod device;
mod stats;
mod ui;
mod config;

use usbmon::{check_usbmon_status, prompt_user_to_load_module, attempt_load_usbmon, print_platform_instructions};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "usbtop-ng")]
#[command(about = "Next-generation USB monitoring tool with real-time bandwidth tracking")]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
    
    /// Refresh rate in milliseconds
    #[arg(short, long, default_value = "1000")]
    refresh: u64,
    
    /// Force run without usbmon (limited functionality)
    #[arg(long)]
    force: bool,
    
    /// Show platform-specific setup instructions
    #[arg(long)]
    setup: bool,
    
    /// Create shell alias for 'usbtop' command
    #[arg(long)]
    create_alias: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
    
    info!("Starting usbtop-ng v{}", env!("CARGO_PKG_VERSION"));
    
    // Show setup instructions if requested
    if cli.setup {
        print_platform_instructions();
        return Ok(());
    }
    
    // Create shell alias if requested
    if cli.create_alias {
        create_shell_alias()?;
        return Ok(());
    }
    
    // Check usbmon status
    let usbmon_status = match check_usbmon_status() {
        Ok(status) => status,
        Err(e) => {
            error!("Failed to check usbmon status: {}", e);
            if !cli.force {
                process::exit(1);
            }
            warn!("Continuing in force mode with limited functionality");
            usbmon::UsbmonStatus {
                module_loaded: false,
                debugfs_mounted: false,
                usbmon_available: false,
                available_buses: Vec::new(),
            }
        }
    };
    
    // Handle usbmon not being available
    if !usbmon_status.usbmon_available && !cli.force {
        if !usbmon_status.module_loaded {
            // Prompt user to load module
            if prompt_user_to_load_module()? {
                if let Err(e) = attempt_load_usbmon() {
                    error!("Failed to load usbmon: {}", e);
                    println!();
                    print_platform_instructions();
                    process::exit(1);
                }
                
                // Re-check status after loading
                let new_status = check_usbmon_status()?;
                if !new_status.usbmon_available {
                    error!("usbmon still not available after loading module");
                    print_platform_instructions();
                    process::exit(1);
                }
                
                info!("usbmon module loaded successfully");
            } else {
                println!("Cannot continue without usbmon. Use --force to run with limited functionality.");
                println!("Run with --setup to see platform-specific instructions.");
                process::exit(1);
            }
        } else if !usbmon_status.debugfs_mounted {
            error!("debugfs is not mounted");
            print_platform_instructions();
            process::exit(1);
        } else {
            error!("usbmon interface not available");
            print_platform_instructions();
            process::exit(1);
        }
    }
    
    // Log available buses
    if !usbmon_status.available_buses.is_empty() {
        info!("Available USB buses: {:?}", usbmon_status.available_buses);
    } else if !cli.force {
        warn!("No USB buses detected");
    }
    
    // Initialize and run the UI
    info!("Starting USB monitoring interface...");
    
    // TODO: Initialize the actual monitoring and UI
    println!("🚀 usbtop-ng starting...");
    println!("📊 Monitoring {} USB buses", usbmon_status.available_buses.len());
    println!("⏱️  Refresh rate: {}ms", cli.refresh);
    println!("📁 Available buses: {:?}", usbmon_status.available_buses);
    
    // For now, just show status and exit
    println!("\n✅ usbtop-ng initialized successfully!");
    println!("🔧 Full monitoring interface coming next...");
    
    Ok(())
}

fn create_shell_alias() -> Result<()> {
    println!("🔗 Creating shell alias for 'usbtop' command...\n");
    
    // Get the current executable path
    let current_exe = env::current_exe()?;
    let exe_path = current_exe.to_string_lossy();
    
    println!("Current executable: {}", exe_path);
    println!("This will create an alias so you can run 'usbtop' instead of 'usbtop-ng'\n");
    
    // Detect shell
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let shell_name = Path::new(&shell).file_name().unwrap().to_string_lossy();
    
    println!("Detected shell: {} ({})", shell_name, shell);
    
    // Determine config file based on shell
    let home = env::var("HOME")?;
    let config_file = match shell_name.as_ref() {
        "bash" => format!("{}/.bashrc", home),
        "zsh" => format!("{}/.zshrc", home),
        "fish" => format!("{}/.config/fish/config.fish", home),
        "tcsh" | "csh" => format!("{}/.cshrc", home),
        _ => format!("{}/.profile", home),
    };
    
    println!("Will add alias to: {}", config_file);
    
    // Ask for confirmation
    print!("\nDo you want to create the alias? (y/N): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if !["y", "yes"].contains(&input.trim().to_lowercase().as_str()) {
        println!("Alias creation cancelled.");
        return Ok(());
    }
    
    // Generate alias command based on shell
    let alias_command = match shell_name.as_ref() {
        "fish" => format!("alias usbtop '{}'", exe_path),
        _ => format!("alias usbtop='{}'", exe_path),
    };
    
    // Check if alias already exists
    if Path::new(&config_file).exists() {
        let content = std::fs::read_to_string(&config_file)?;
        if content.contains("alias usbtop") {
            println!("⚠️  An 'usbtop' alias already exists in {}!", config_file);
            print!("Do you want to replace it? (y/N): ");
            io::stdout().flush()?;
            
            let mut replace_input = String::new();
            io::stdin().read_line(&mut replace_input)?;
            
            if !["y", "yes"].contains(&replace_input.trim().to_lowercase().as_str()) {
                println!("Alias creation cancelled.");
                return Ok(());
            }
        }
    }
    
    // Add the alias to the config file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_file)?;
    
    writeln!(file, "\n# usbtop-ng alias (added by usbtop-ng --create-alias)")?;
    writeln!(file, "{}", alias_command)?;
    
    println!("✅ Successfully added alias to {}", config_file);
    println!("\nTo use the alias in your current session, run:");
    println!("  source {}", config_file);
    println!("\nOr start a new terminal session.");
    println!("\nYou can now run: usbtop");
    
    Ok(())
}
