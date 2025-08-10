use chrono::{DateTime, Utc};
use std::time::Instant;

use crate::usbmon::parser::UsbSpeed;
use crate::stats::BandwidthStats;

pub mod manager;

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub bus_id: u8,
    pub device_id: u8,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub vendor: Option<String>,
    pub product: Option<String>,
    pub serial: Option<String>,
    pub speed: UsbSpeed,
    pub bandwidth_stats: BandwidthStats,
    pub is_disconnected: bool,
    pub disconnect_time: Option<Instant>,
    pub last_seen: Instant,
}

impl UsbDevice {
    pub fn new(bus_id: u8, device_id: u8) -> Self {
        Self {
            bus_id,
            device_id,
            vendor_id: None,
            product_id: None,
            vendor: None,
            product: None,
            serial: None,
            speed: UsbSpeed::Unknown,
            bandwidth_stats: BandwidthStats::new(),
            is_disconnected: false,
            disconnect_time: None,
            last_seen: Instant::now(),
        }
    }
    
    pub fn update_from_sysfs(&mut self) -> Result<(), std::io::Error> {
        #[cfg(target_os = "linux")]
        {
            self.update_linux_device_info()
        }
        
        #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
        {
            self.update_bsd_device_info()
        }
        
        #[cfg(target_os = "macos")]
        {
            self.update_macos_device_info()
        }
    }
    
    #[cfg(target_os = "linux")]
    fn update_linux_device_info(&mut self) -> Result<(), std::io::Error> {
        use std::fs;
        use std::path::Path;
        
        // Find device path in sysfs
        let sysfs_path = format!("/sys/bus/usb/devices/{}-{}", self.bus_id, self.device_id);
        if !Path::new(&sysfs_path).exists() {
            // Try alternative path patterns
            let alt_paths = [
                format!("/sys/bus/usb/devices/usb{}/{}-{}", self.bus_id, self.bus_id, self.device_id),
                format!("/sys/bus/usb/devices/{}", self.device_id),
            ];
            
            let mut found_path = None;
            for path in &alt_paths {
                if Path::new(path).exists() {
                    found_path = Some(path.clone());
                    break;
                }
            }
            
            if found_path.is_none() {
                return Ok(()); // Device not found in sysfs, skip
            }
        }
        
        // Read device attributes
        if let Ok(speed_str) = fs::read_to_string(format!("{}/speed", sysfs_path)) {
            self.speed = UsbSpeed::from_speed_str(speed_str.trim());
        }
        
        if let Ok(vendor_str) = fs::read_to_string(format!("{}/idVendor", sysfs_path)) {
            if let Ok(vendor_id) = u16::from_str_radix(vendor_str.trim(), 16) {
                self.vendor_id = Some(vendor_id);
            }
        }
        
        if let Ok(product_str) = fs::read_to_string(format!("{}/idProduct", sysfs_path)) {
            if let Ok(product_id) = u16::from_str_radix(product_str.trim(), 16) {
                self.product_id = Some(product_id);
            }
        }
        
        if let Ok(manufacturer) = fs::read_to_string(format!("{}/manufacturer", sysfs_path)) {
            self.vendor = Some(manufacturer.trim().to_string());
        }
        
        if let Ok(product) = fs::read_to_string(format!("{}/product", sysfs_path)) {
            self.product = Some(product.trim().to_string());
        }
        
        if let Ok(serial) = fs::read_to_string(format!("{}/serial", sysfs_path)) {
            self.serial = Some(serial.trim().to_string());
        }
        
        Ok(())
    }
    
    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    fn update_bsd_device_info(&mut self) -> Result<(), std::io::Error> {
        // For BSD systems, we might use usbconfig or similar utilities
        // This is a placeholder implementation
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn update_macos_device_info(&mut self) -> Result<(), std::io::Error> {
        // For macOS, we might use ioreg or system_profiler
        // This is a placeholder implementation
        Ok(())
    }
    
    pub fn mark_disconnected(&mut self) {
        if !self.is_disconnected {
            self.is_disconnected = true;
            self.disconnect_time = Some(Instant::now());
        }
    }
    
    pub fn should_remove(&self) -> bool {
        if let Some(disconnect_time) = self.disconnect_time {
            // Remove after 5 seconds of being disconnected
            disconnect_time.elapsed().as_secs() > 5
        } else {
            false
        }
    }
    
    pub fn update_activity(&mut self) {
        self.last_seen = Instant::now();
        if self.is_disconnected {
            self.is_disconnected = false;
            self.disconnect_time = None;
        }
    }
    
    /// Calculate the percentage of device bandwidth being utilized
    /// Uses practical bandwidth (accounting for protocol overhead)
    pub fn get_busy_percentage(&self) -> f64 {
        let max_bandwidth = self.speed.to_practical_bytes_per_second();
        self.bandwidth_stats.get_utilization_percentage(max_bandwidth)
    }
    
    /// Calculate the percentage of device bandwidth being utilized (theoretical max)
    /// Uses raw theoretical bandwidth (no overhead consideration)
    pub fn get_busy_percentage_theoretical(&self) -> f64 {
        let max_bandwidth = self.speed.to_bytes_per_second();
        self.bandwidth_stats.get_utilization_percentage(max_bandwidth)
    }
    
    /// Get the maximum speed capability of this device from USB descriptors
    /// Note: This is a heuristic approach since actual device capability info
    /// may not always be available in sysfs
    #[cfg(target_os = "linux")]
    pub fn get_device_max_capability(&self) -> UsbSpeed {
        use std::fs;
        use std::path::Path;
        
        // Try to read bcdUSB version which indicates device capability
        let sysfs_path = format!("/sys/bus/usb/devices/{}-{}", self.bus_id, self.device_id);
        
        if let Ok(bcd_device) = fs::read_to_string(format!("{}/bcdDevice", sysfs_path)) {
            // Parse bcdDevice to infer capabilities (this is heuristic)
            if let Ok(bcd_val) = u16::from_str_radix(bcd_device.trim(), 16) {
                // Modern devices (post-2010) that support USB 3.0+ often have bcdDevice >= 0x300
                if bcd_val >= 0x0300 {
                    return UsbSpeed::SuperSpeed;
                }
            }
        }
        
        // Fallback: Check for high-speed capable descriptors
        if let Ok(bmaxpkts0) = fs::read_to_string(format!("{}/bMaxPacketSize0", sysfs_path)) {
            if let Ok(max_packet) = bmaxpkts0.trim().parse::<u16>() {
                // High-speed devices can have larger max packet sizes
                if max_packet >= 64 {
                    return UsbSpeed::High;
                } else if max_packet == 8 {
                    return UsbSpeed::Low;
                } else {
                    return UsbSpeed::Full;
                }
            }
        }
        
        // If we can't determine capability, assume it matches current speed
        self.speed.clone()
    }
    
    #[cfg(not(target_os = "linux"))]
    pub fn get_device_max_capability(&self) -> UsbSpeed {
        // For non-Linux systems, we can't easily detect device capabilities
        // so assume current speed is max capability
        self.speed.clone()
    }
    
    /// Check if device is potentially limited by bus speed
    /// Returns Some(device_capability) if device could run faster on a different bus
    pub fn check_speed_mismatch(&self, bus_speed: &UsbSpeed) -> Option<UsbSpeed> {
        let device_capability = self.get_device_max_capability();
        
        // If device capability is higher than bus speed, there's a potential mismatch
        if device_capability.to_mbps() > bus_speed.to_mbps() && 
           device_capability.to_mbps() > self.speed.to_mbps() {
            Some(device_capability)
        } else {
            None
        }
    }
    
    /// Get a visual indicator for speed capability issues
    pub fn get_speed_indicator(&self, bus_speed: &UsbSpeed) -> SpeedIndicator {
        if let Some(capable_speed) = self.check_speed_mismatch(bus_speed) {
            SpeedIndicator::LimitedByBus(capable_speed)
        } else if self.speed.to_mbps() > 0.0 && self.get_busy_percentage() > 80.0 {
            SpeedIndicator::HighUtilization
        } else {
            SpeedIndicator::Normal
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpeedIndicator {
    Normal,
    HighUtilization,
    LimitedByBus(UsbSpeed), // Contains the speed the device is capable of
}

impl SpeedIndicator {
    /// Get a visual symbol for the speed indicator
    pub fn get_symbol(&self) -> &'static str {
        match self {
            SpeedIndicator::Normal => "",
            SpeedIndicator::HighUtilization => "⚡",
            SpeedIndicator::LimitedByBus(_) => "🔺",
        }
    }
    
    /// Get a color code for the indicator
    pub fn get_color(&self) -> (u8, u8, u8) {
        match self {
            SpeedIndicator::Normal => (128, 128, 128),        // Gray
            SpeedIndicator::HighUtilization => (255, 165, 0), // Orange
            SpeedIndicator::LimitedByBus(_) => (255, 255, 0), // Yellow
        }
    }
    
    /// Get a description of the indicator
    pub fn get_description(&self) -> String {
        match self {
            SpeedIndicator::Normal => "Normal operation".to_string(),
            SpeedIndicator::HighUtilization => "High bandwidth utilization".to_string(),
            SpeedIndicator::LimitedByBus(capable_speed) => {
                format!("Device capable of {} but limited by bus speed", 
                       format_speed(capable_speed))
            }
        }
    }
}

/// Format USB speed for display
pub fn format_speed(speed: &UsbSpeed) -> String {
    match speed {
        UsbSpeed::Low => "1.5 Mbps (Low Speed)".to_string(),
        UsbSpeed::Full => "12 Mbps (Full Speed)".to_string(), 
        UsbSpeed::High => "480 Mbps (High Speed)".to_string(),
        UsbSpeed::SuperSpeed => "5 Gbps (SuperSpeed)".to_string(),
        UsbSpeed::SuperSpeedPlus => "10+ Gbps (SuperSpeed+)".to_string(),
        UsbSpeed::Unknown => "Unknown".to_string(),
    }
}

/// Format bandwidth utilization percentage for display
pub fn format_busy_percentage(percentage: f64) -> String {
    format!("{:5.1}%", percentage)
}

/// Format bandwidth in human-readable units
pub fn format_bandwidth(bps: f64) -> String {
    if bps >= 1_000_000_000.0 {
        format!("{:.1} GB/s", bps / 1_000_000_000.0)
    } else if bps >= 1_000_000.0 {
        format!("{:.1} MB/s", bps / 1_000_000.0)
    } else if bps >= 1_000.0 {
        format!("{:.1} KB/s", bps / 1_000.0)
    } else {
        format!("{:.0} B/s", bps)
    }
}