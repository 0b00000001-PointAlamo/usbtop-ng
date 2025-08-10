# usbtop-ng

usbtop-ng is a next-generation USB traffic monitoring tool, reimagined in Rust for speed, safety, and modern systems.

Inspired by the original [usbtop](https://github.com/aguinet/usbtop), usbtop-ng is an **independent reimplementation**.  
It does **not** share code with the original project and is **not affiliated with or endorsed by** its authors.

## ✨ Features

- Real-time USB traffic statistics with **%busy** calculations for devices and buses
- **Speed capability mismatch detection** - shows when devices could run faster  
- Lightweight, terminal-friendly interface
- Rust-powered performance and safety
- Cross-platform support (Linux, *BSD, macOS — Windows WIP)
- Low resource footprint
- **Visual indicators** for high utilization and speed limitations

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/usbtop-ng.git
cd usbtop-ng

# Build and install
cargo build --release
sudo cp target/release/usbtop-ng /usr/local/bin/
```

### Creating a Shell Alias

For convenience, you can create an alias so you can run `usbtop` instead of `usbtop-ng`:

```bash
# Let usbtop-ng create the alias for you
usbtop-ng --create-alias

# Or manually add to your shell config (~/.bashrc, ~/.zshrc, etc.)
alias usbtop='usbtop-ng'
```

## 🚀 Usage

```bash
usbtop-ng
# or if you created the alias:
usbtop
```

Press `q` to quit.  
Run with `--help` to see all options.

### Command Line Options

```
usbtop-ng [OPTIONS]

Options:
  -v, --verbose            Enable verbose logging
  -c, --config <CONFIG>    Configuration file path
  -r, --refresh <REFRESH>  Refresh rate in milliseconds [default: 1000]
      --force              Force run without usbmon (limited functionality)
      --setup              Show platform-specific setup instructions
      --create-alias       Create shell alias for 'usbtop' command
  -h, --help               Print help
  -V, --version            Print version
```

### %busy Display Features

- **Device %busy**: Shows bandwidth utilization percentage for each USB device
- **Bus %busy**: Shows total bandwidth utilization for each USB bus  
- **Speed indicators**: Visual symbols for devices that could run faster
  - ⚡ High utilization (>80% bandwidth usage)
  - 🔺 Limited by bus speed (device capable of faster speed)

## 🛠 Development

Requirements:
- Rust (latest stable)
- libusb (development headers)

Build:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

## 📄 License

This project is licensed under the **BSD 3-Clause License**.  
You are free to use, modify, and distribute this code, provided you include the original copyright and license notice in any copies or substantial portions.

See [LICENSE](LICENSE) for full details.

## 🙏 Acknowledgments

- Inspired by the original [usbtop](https://github.com/aguinet/usbtop) by Antoine Guinet.
- Thanks to the Rust community for making systems programming safer and fun.