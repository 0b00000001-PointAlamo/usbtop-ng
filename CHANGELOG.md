# Changelog

All notable changes to ng-usbtop will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of ng-usbtop
- Real-time USB bandwidth monitoring using usbmon interface
- Interactive terminal UI with ratatui
- Cross-platform support (Linux, BSD, macOS)
- USB speed color coding system
- Device disconnection tracking with 5-second grey display
- Bandwidth history visualization with 60-second sliding window
- Automatic usbmon kernel module detection and loading
- Device metadata extraction from sysfs
- Configuration file support (TOML format)
- Comprehensive help system and keyboard navigation
- Command-line interface with multiple options
- Platform-specific setup instructions
- Integration tests and unit test coverage

### Technical Details
- Built with Rust 2021 edition
- Async I/O using Tokio runtime
- Terminal UI powered by ratatui and crossterm
- USB packet parsing for both binary and text usbmon formats
- Multi-threaded architecture with proper error handling
- Modular codebase with clear separation of concerns

## [0.1.0] - 2024-07-30

### Added
- Initial release of ng-usbtop
- Core USB monitoring functionality
- Terminal user interface
- Cross-platform compatibility layer
- Documentation and build system

---

## Release Notes

### Version 0.1.0

This is the initial release of ng-usbtop, a next-generation USB monitoring tool designed to replace and enhance the original usbtop utility.

#### Key Features
- **Real-time monitoring**: Live USB bandwidth tracking with sub-second updates
- **Rich terminal UI**: Colorful, interactive interface inspired by modern system monitors
- **Cross-platform**: Native support for Linux, BSD variants, and macOS
- **Smart detection**: Automatic usbmon module detection and setup assistance
- **Visual feedback**: Color-coded USB speeds and device status indicators
- **Historical data**: Bandwidth graphs with configurable time windows

#### System Requirements
- Rust 1.70 or later (for building from source)
- Linux: usbmon kernel module and debugfs
- BSD: Native USB monitoring interfaces
- macOS: Limited functionality (device enumeration only)

#### Installation
```bash
cargo install ng-usbtop
```

Or download pre-built binaries from the releases page.

#### Usage
```bash
# Basic usage (will prompt for usbmon setup if needed)
ng-usbtop

# Show help
ng-usbtop --help

# Platform-specific setup instructions
ng-usbtop --setup
```

#### Known Limitations
- Requires root privileges on most systems
- macOS support is limited due to lack of usbmon equivalent
- Some USB controllers may not be fully supported
- Binary usbmon format parsing is still being refined

#### Roadmap
- Configuration file support improvements
- Additional platform-specific optimizations  
- Enhanced device filtering and search
- Export functionality for bandwidth data
- Plugin system for custom monitors
- Network-based monitoring for remote systems

---

For detailed technical information, see the [README.md](README.md) and [documentation](docs/).

Report issues and feature requests on [GitHub Issues](https://github.com/your-repo/ng-usbtop/issues).