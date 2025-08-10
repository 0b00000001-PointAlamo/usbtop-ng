# usbtop-ng

usbtop-ng is a next-generation USB traffic monitoring tool, reimagined in Rust for speed, safety, and modern systems.

Inspired by the original [usbtop](https://github.com/aguinet/usbtop), usbtop-ng is an **independent reimplementation**.  
It does **not** share code with the original project and is **not affiliated with or endorsed by** its authors.

## ✨ Features

- Real-time USB traffic statistics
- Lightweight, terminal-friendly interface
- Rust-powered performance and safety
- Cross-platform support (Linux, *BSD, macOS — Windows WIP)
- Low resource footprint

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/usbtop-ng.git
cd usbtop-ng

# Build and install
cargo build --release
sudo cp target/release/usbtop-ng /usr/local/bin/
```

## 🚀 Usage

```bash
usbtop-ng
```

Press `q` to quit.  
Run with `--help` to see all options.

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
