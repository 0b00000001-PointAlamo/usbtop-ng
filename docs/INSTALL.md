# Installation Guide

This guide covers various methods to install and set up ng-usbtop on different operating systems.

## Table of Contents

- [System Requirements](#system-requirements)
- [Installation Methods](#installation-methods)
- [Platform-Specific Setup](#platform-specific-setup)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Uninstalling](#uninstalling)

## System Requirements

### Hardware
- x86_64 or ARM64 architecture
- Minimum 100 MB disk space
- At least 512 MB RAM

### Software
- **Linux**: Kernel 2.6.35+ with usbmon support
- **FreeBSD**: 12.0+ with USB support
- **OpenBSD**: 6.5+ with USB support  
- **NetBSD**: 8.0+ with USB support
- **macOS**: 10.15+ (limited functionality)

## Installation Methods

### Method 1: Pre-built Binaries (Recommended)

Download the latest binary for your platform from the [releases page](https://github.com/your-repo/ng-usbtop/releases).

```bash
# Linux x86_64
curl -L https://github.com/your-repo/ng-usbtop/releases/latest/download/ng-usbtop-linux-x86_64.tar.gz | tar xz
sudo cp ng-usbtop /usr/local/bin/

# macOS (via Homebrew)
brew install ng-usbtop

# macOS (manual)
curl -L https://github.com/your-repo/ng-usbtop/releases/latest/download/ng-usbtop-macos.tar.gz | tar xz
sudo cp ng-usbtop /usr/local/bin/
```

### Method 2: Cargo (Rust Package Manager)

```bash
# Install from crates.io
cargo install ng-usbtop

# Install from git (latest development version)
cargo install --git https://github.com/your-repo/ng-usbtop.git
```

### Method 3: Build from Source

#### Prerequisites
- Rust 1.70 or later ([install from rustup.rs](https://rustup.rs/))
- Git

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/your-repo/ng-usbtop.git
cd ng-usbtop

# Build release version
cargo build --release

# Install to system (optional)
sudo cp target/release/ng-usbtop /usr/local/bin/

# Or install via cargo
cargo install --path .
```

### Method 4: Package Managers

#### Arch Linux (AUR)
```bash
# Using yay
yay -S ng-usbtop

# Using paru
paru -S ng-usbtop
```

#### Ubuntu/Debian (PPA)
```bash
sudo add-apt-repository ppa:ng-usbtop/stable
sudo apt update
sudo apt install ng-usbtop
```

#### Fedora/RHEL
```bash
sudo dnf copr enable ng-usbtop/stable
sudo dnf install ng-usbtop
```

#### FreeBSD Ports
```bash
sudo pkg install ng-usbtop
```

## Platform-Specific Setup

### Linux

#### 1. Kernel Module Setup
```bash
# Check if usbmon is available
ls /lib/modules/$(uname -r)/kernel/drivers/usb/mon/

# Load usbmon module
sudo modprobe usbmon

# Verify module is loaded
lsmod | grep usbmon

# Make persistent (optional)
echo "usbmon" | sudo tee -a /etc/modules
```

#### 2. Debugfs Setup
```bash
# Check if debugfs is mounted
mount | grep debugfs

# Mount debugfs if needed
sudo mount -t debugfs none /sys/kernel/debug

# Make persistent (add to /etc/fstab)
echo "none /sys/kernel/debug debugfs defaults 0 0" | sudo tee -a /etc/fstab
```

#### 3. Permissions
```bash
# Option 1: Run as root (simplest)
sudo ng-usbtop

# Option 2: Add user to appropriate group
sudo usermod -a -G plugdev $USER    # Debian/Ubuntu
sudo usermod -a -G wheel $USER       # Arch/RHEL
sudo usermod -a -G operator $USER    # Some distributions

# Log out and back in for group changes to take effect
```

#### 4. Distribution-Specific Notes

**Ubuntu/Debian:**
```bash
# Install additional dependencies if building from source
sudo apt install build-essential pkg-config

# usbmon is usually built into the kernel
```

**Arch Linux:**
```bash
# usbmon should be available as a module
sudo modprobe usbmon
```

**RHEL/CentOS/Fedora:**
```bash
# May need to install kernel modules
sudo dnf install kernel-modules-extra
sudo modprobe usbmon
```

### FreeBSD

#### 1. Enable USB Support
```bash
# Check USB support in kernel
kldstat | grep usb

# Load USB modules if needed
sudo kldload usb
sudo kldload uhci  # or ohci/ehci/xhci as appropriate
```

#### 2. Device Permissions
```bash
# Check USB devices
usbconfig

# Set permissions for USB devices
sudo sysctl hw.usb.template=3
```

#### 3. Install ng-usbtop
```bash
# Via pkg
sudo pkg install ng-usbtop

# Via ports
cd /usr/ports/sysutils/ng-usbtop
sudo make install clean
```

### OpenBSD

#### 1. USB Support
```bash
# USB support is usually built into GENERIC kernel
dmesg | grep -i usb

# Check USB devices
usbdevs
```

#### 2. Permissions
```bash
# Add user to wheel group
sudo usermod -G wheel $USER
```

### NetBSD

#### 1. USB Support
```bash
# Check USB support
dmesg | grep -i usb

# List USB devices
usbdevs
```

#### 2. Install from pkgsrc
```bash
cd /usr/pkgsrc/sysutils/ng-usbtop
sudo make install
```

### macOS

#### 1. Install via Homebrew
```bash
brew install ng-usbtop
```

#### 2. Manual Installation
```bash
# Download and install
curl -L https://github.com/your-repo/ng-usbtop/releases/latest/download/ng-usbtop-macos.tar.gz | tar xz
sudo cp ng-usbtop /usr/local/bin/
```

#### 3. Limitations
- No real-time USB monitoring (macOS doesn't have usbmon)
- Static device information only
- Use `--force` flag to run with limited functionality

## Configuration

### Configuration File

Create a configuration file at `~/.config/ng-usbtop/config.toml`:

```toml
[display]
refresh_rate = 1000  # milliseconds
show_disconnected_time = 5  # seconds

[monitoring]
history_window = 60  # seconds
binary_mode = true   # prefer binary usbmon format

[colors]
primary = "#00BFFF"
secondary = "#FF8C00"
accent = "#32CD32"

[advanced]
max_devices = 1000
packet_buffer_size = 4096
```

### Environment Variables

```bash
# Disable colors
export NO_COLOR=1

# Set log level
export RUST_LOG=info

# Custom config path
export NG_USBTOP_CONFIG=/path/to/config.toml
```

## Verification

### Test Installation
```bash
# Check version
ng-usbtop --version

# Show help
ng-usbtop --help

# Test with limited functionality
ng-usbtop --force

# Check platform-specific setup
ng-usbtop --setup
```

### Verify USB Monitoring
```bash
# On Linux, check usbmon access
sudo ls -la /sys/kernel/debug/usb/usbmon/

# Test reading usbmon (should show USB activity)
sudo cat /sys/kernel/debug/usb/usbmon/0t | head -10
```

## Troubleshooting

### Common Issues

#### "usbmon module not loaded"
```bash
# Solution 1: Load module
sudo modprobe usbmon

# Solution 2: Check if built into kernel
grep -i usbmon /boot/config-$(uname -r)

# Solution 3: Rebuild kernel with usbmon support
```

#### "Permission denied"
```bash
# Solution 1: Run as root
sudo ng-usbtop

# Solution 2: Fix debugfs permissions
sudo chmod o+rx /sys/kernel/debug
sudo chmod o+rx /sys/kernel/debug/usb
sudo chmod o+r /sys/kernel/debug/usb/usbmon/*

# Solution 3: Use udev rules
echo 'SUBSYSTEM=="usbmon", GROUP="plugdev", MODE="0640"' | sudo tee /etc/udev/rules.d/50-usbmon.rules
sudo udevadm control --reload-rules
```

#### "No USB buses detected"
```bash
# Check if any USB controllers exist
lspci | grep -i usb

# Check if USB devices are visible
lsusb

# Verify debugfs mount
mount | grep debugfs
```

#### Performance Issues
```bash
# Reduce refresh rate
ng-usbtop --refresh 2000

# Check system resources
top -p $(pgrep ng-usbtop)

# Enable debug logging
RUST_LOG=debug ng-usbtop --verbose
```

### Getting Help

If you encounter issues:

1. Check the [troubleshooting documentation](TROUBLESHOOTING.md)
2. Search [existing issues](https://github.com/your-repo/ng-usbtop/issues)
3. Create a [new issue](https://github.com/your-repo/ng-usbtop/issues/new) with:
   - Operating system and version
   - ng-usbtop version (`ng-usbtop --version`)
   - Error message and log output
   - Steps to reproduce

## Uninstalling

### Remove Binary
```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/ng-usbtop

# If installed via cargo
cargo uninstall ng-usbtop
```

### Remove Configuration
```bash
# Remove config directory
rm -rf ~/.config/ng-usbtop

# Remove any systemd services (if created)
systemctl --user disable ng-usbtop
rm ~/.config/systemd/user/ng-usbtop.service
```

### Package Manager Removal
```bash
# Arch Linux
sudo pacman -R ng-usbtop

# Ubuntu/Debian
sudo apt remove ng-usbtop

# Fedora
sudo dnf remove ng-usbtop

# FreeBSD
sudo pkg delete ng-usbtop

# macOS (Homebrew)
brew uninstall ng-usbtop
```

---

For additional help, see the [main README](../README.md) or visit our [documentation](https://github.com/your-repo/ng-usbtop/wiki).