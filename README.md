# 📱 Simbox

> A powerful and user-friendly CLI tool for managing iOS simulators

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

Simbox makes it easy to list, search, and launch iOS simulators from your terminal. No more hunting through Xcode or remembering complex commands!

## ✨ Features

- 🔍 **Interactive Mode**: Fuzzy search through your simulators
- 📋 **List View**: See all available simulators with their status
- 🎯 **Smart Filtering**: Filter simulators by type (iPhone, iPad)
- 🚀 **Quick Launch**: Boot simulators directly by name
- 🎨 **Beautiful Interface**: Color-coded output for better visibility
- ⚡️ **Fast & Efficient**: Written in Rust for maximum performance

## 🚀 Installation

```bash
cargo install simbox
```

## 🎯 Usage

### Interactive Mode (Recommended)

Launch the interactive selector with fuzzy search:
```bash
simbox -i
# or
simbox --interactive
```

### List All Simulators

View all available simulators:
```bash
simbox --list
```

### Filter Simulators

Show only specific device types:
```bash
simbox --list iphone    # Show only iPhones
simbox --list ipad      # Show only iPads
simbox --list pro       # Show only Pro devices
```

### Boot Specific Simulator

Launch a simulator by name:
```bash
simbox --boot "iPhone 15 Pro"
```

## 🎨 Output Format

Simulators are displayed with color-coded information:
- Device Name (white)
- iOS Version (cyan)
- Status:
  - 🟢 Booted (green)
  - 🔴 Shutdown (red)

Example output:
```
iPhone 15 Pro (iOS 17.2) (Shutdown)
iPhone 16 Pro Max (iOS 18.2) (Booted)
```

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org)
- Uses [inquire](https://docs.rs/inquire) for interactive selection
- Uses [clap](https://docs.rs/clap) for CLI argument parsing
