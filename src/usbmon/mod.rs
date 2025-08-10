use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, anyhow};
use log::{info, warn, debug};

pub mod reader;
pub mod parser;

#[derive(Debug, Clone)]
pub struct UsbmonStatus {
    pub module_loaded: bool,
    pub debugfs_mounted: bool,
    pub usbmon_available: bool,
    pub available_buses: Vec<u8>,
}

pub fn check_usbmon_status() -> Result<UsbmonStatus> {
    debug!("Checking usbmon kernel module status");
    
    let module_loaded = is_usbmon_module_loaded()?;
    let debugfs_mounted = is_debugfs_mounted()?;
    let usbmon_available = debugfs_mounted && check_usbmon_debugfs_exists()?;
    let available_buses = if usbmon_available {
        get_available_buses()?
    } else {
        Vec::new()
    };

    Ok(UsbmonStatus {
        module_loaded,
        debugfs_mounted,
        usbmon_available,
        available_buses,
    })
}

fn is_usbmon_module_loaded() -> Result<bool> {
    #[cfg(target_os = "linux")]
    {
        let modules = fs::read_to_string("/proc/modules")?;
        Ok(modules.lines().any(|line| line.starts_with("usbmon ")))
    }
    
    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        // BSD systems may have USB monitoring built-in or use different mechanisms
        let output = Command::new("kldstat")
            .output()
            .map_err(|e| anyhow!("Failed to run kldstat: {}", e))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains("usb") || stdout.contains("ugen"))
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS doesn't have usbmon, but we can still detect USB via system_profiler
        warn!("macOS does not support usbmon kernel module");
        Ok(false)
    }
}

fn is_debugfs_mounted() -> Result<bool> {
    #[cfg(target_os = "linux")]
    {
        let mounts = fs::read_to_string("/proc/mounts")?;
        Ok(mounts.lines().any(|line| {
            line.contains("debugfs") && line.contains("/sys/kernel/debug")
        }))
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        // Non-Linux systems use different paths
        Ok(true)
    }
}

fn check_usbmon_debugfs_exists() -> Result<bool> {
    #[cfg(target_os = "linux")]
    {
        Ok(Path::new("/sys/kernel/debug/usb/usbmon").exists())
    }
    
    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        // BSD systems may use /dev/ugen* or similar
        Ok(Path::new("/dev").exists())
    }
    
    #[cfg(target_os = "macos")]
    {
        Ok(false)
    }
}

fn get_available_buses() -> Result<Vec<u8>> {
    #[cfg(target_os = "linux")]
    {
        let mut buses = Vec::new();
        
        if let Ok(entries) = fs::read_dir("/sys/kernel/debug/usb/usbmon") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let filename = entry.file_name();
                    let filename_str = filename.to_string_lossy();
                    
                    // Look for files like "0u", "1u", "2u", etc.
                    if filename_str.ends_with('u') && filename_str.len() >= 2 {
                        if let Ok(bus_num) = filename_str[0..filename_str.len()-1].parse::<u8>() {
                            buses.push(bus_num);
                        }
                    }
                }
            }
        }
        
        buses.sort();
        Ok(buses)
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        // For non-Linux systems, we'll implement bus discovery differently
        Ok(vec![0])
    }
}

pub fn prompt_user_to_load_module() -> Result<bool> {
    use std::io::{self, Write};
    
    println!("❌ usbmon kernel module is not loaded!");
    println!();
    println!("usbtop-ng requires the usbmon kernel module to monitor USB traffic.");
    println!("This module is safe and provides read-only access to USB bus activity.");
    println!();
    println!("To load the module, run:");
    println!("  sudo modprobe usbmon");
    println!();
    println!("You may also need to mount debugfs if not already mounted:");
    println!("  sudo mount -t debugfs none /sys/kernel/debug");
    println!();
    print!("Would you like usbtop-ng to attempt loading the module? (y/N): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}

pub fn attempt_load_usbmon() -> Result<()> {
    info!("Attempting to load usbmon kernel module");
    
    #[cfg(target_os = "linux")]
    {
        // Try to load usbmon module
        let output = Command::new("sudo")
            .args(&["modprobe", "usbmon"])
            .output()
            .map_err(|e| anyhow!("Failed to run modprobe: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to load usbmon module: {}", stderr));
        }
        
        // Try to mount debugfs if needed
        if !is_debugfs_mounted()? {
            info!("Attempting to mount debugfs");
            let output = Command::new("sudo")
                .args(&["mount", "-t", "debugfs", "none", "/sys/kernel/debug"])
                .output()
                .map_err(|e| anyhow!("Failed to mount debugfs: {}", e))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!("Failed to mount debugfs (may already be mounted): {}", stderr);
            }
        }
        
        Ok(())
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Err(anyhow!("Automatic module loading not supported on this platform"))
    }
}

pub fn print_platform_instructions() {
    #[cfg(target_os = "linux")]
    {
        println!("📋 Linux Setup Instructions:");
        println!("1. Load the usbmon kernel module:");
        println!("   sudo modprobe usbmon");
        println!("2. Ensure debugfs is mounted:");
        println!("   sudo mount -t debugfs none /sys/kernel/debug");
        println!("3. Run usbtop-ng as root or add your user to the appropriate group");
    }
    
    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        println!("📋 BSD Setup Instructions:");
        println!("1. Ensure USB support is enabled in kernel");
        println!("2. Check available USB devices with: usbconfig");
        println!("3. Run usbtop-ng with appropriate permissions");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("📋 macOS Setup Instructions:");
        println!("⚠️  Note: macOS does not have usbmon equivalent");
        println!("Consider using alternative tools like:");
        println!("- USB Prober (part of Additional Tools for Xcode)");
        println!("- system_profiler SPUSBDataType");
        println!("- ioreg -p IOUSB");
    }
}