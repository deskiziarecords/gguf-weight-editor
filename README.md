# Weight Editor

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![eframe](https://img.shields.io/badge/GUI-eframe%2Fegui-green.svg)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)]()

> A lightweight, privacy-focused weight tracking application built with Rust.  
> Your data stays on your device. Always.



---

##  Features

-  **Simple Entry Management** — Add, edit, and delete weight records with ease
-  **Automatic Persistence** — Data saved as JSON with automatic backups
-  **Built-in Statistics** — Track min/max/average weight and total progress
-  **CSV Export** — Export your data for analysis in Excel, Google Sheets, or any spreadsheet
-  **Dark & Light Themes** — Toggle between comfortable viewing modes
-  **Keyboard Shortcuts** — Press `Enter` to quickly submit entries
-  **Quick Date Entry** — One-click "Today" button for daily logging
-  **Privacy First** — No cloud, no accounts, no telemetry. Your data is yours.
-  **Cross-Platform** — Native GUI that runs on Windows, macOS, and Linux
-  **Lightweight** — Fast startup, small memory footprint, optimized release builds

---

##  Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- A C compiler (for some dependencies)
- System GUI libraries (usually pre-installed)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/gguff-weight-editor.git
cd gguff-weight-editor

# Build the optimized release binary
cargo build --release
# Run it!
cargo run --release
```
The compiled binary will be located at:
Linux/macOS: target/release/gguff-weight-editor
Windows: target/release/gguff-weight-editor.exe
System Dependencies (Linux)
On some Linux distributions, you may need:

```bash
# Ubuntu/Debian
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
                 libxkbcommon-dev libgtk-3-dev

# Fedora
sudo dnf install libxcb-devel gtk3-devel

# Arch
sudo pacman -S libxcb gtk3
```

---
Usage
Adding an Entry
Enter a date in YYYY-MM-DD format (or click 📅 Today)
Enter your weight in kilograms
Click ➕ Add Entry or press Enter
Editing an Entry
Click ✏️ Edit on any row
Modify the date or weight
Click ✓ Apply Edit or ❌ Cancel
Viewing Statistics
Click  Toggle Stats to reveal:
Total entries
First and latest measurements
Minimum, maximum, and average weight
Total weight change (color-coded: 🟢 loss, 🔴 gain)
Exporting Data
Click  Export CSV to save your entries as a spreadsheet-compatible file.
 Data Storage
GGUFF Weight Editor stores your data in a platform-appropriate location:
Platform
Path
Windows
%LOCALAPPDATA%\gguff-weight-editor\weights.json
macOS
~/Library/Application Support/gguff-weight-editor/weights.json
Linux
~/.local/share/gguff-weight-editor/weights.json
A backup file (weights.json.bak) is automatically created before each save.
 Building from Source
Development Build
bash



cargo run
Optimized Release Build
bash



cargo build --release
The release profile is configured for:
Maximum size optimization (opt-level = "z")
Link-time optimization (LTO)
Symbol stripping
Single codegen unit
Project Structure



gguff-weight-editor/
├── Cargo.toml           # Dependencies and build configuration
├── README.md            # This file
├── LICENSE              # MIT License
├── src/
│   └── main.rs          # Application source code
└── docs/
    └── screenshot.png   # UI screenshot (optional)
 Roadmap
 Weight Chart Visualization — Plot weight over time with plotters
 CSV Import — Load existing data from spreadsheets
 Multiple Profiles — Track weight for family members
 Goal Tracking — Set target weights and see progress
 Mobile Build — Compile for Android/iOS via egui
 Localization — Multi-language support
 Advanced Analytics — BMI, trends, moving averages
Have an idea? Open an issue or submit a PR!
## Contributing
Contributions are welcome and encouraged! This project is part of the OpenΦ initiative, dedicated to building useful tools for the community.
How to Contribute
Fork the repository
Create a feature branch (git checkout -b feature/amazing-feature)
Commit your changes (git commit -m 'Add amazing feature')
Push to the branch (git push origin feature/amazing-feature)
Open a Pull Request
Code Style
Follow standard Rust formatting (cargo fmt)
Run clippy before submitting (cargo clippy -- -D warnings)
Write clear commit messages
Add tests for new functionality when possible
Ideas for Contributors
Improve error messages
Add unit tests
Enhance the UI/UX
Write documentation
Translate the interface
Package for different platforms
📜 License
This project is licensed under the MIT License — see the LICENSE file for details.



MIT License

Copyright (c) 2026 OpenΦ Community

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software...
 Acknowledgments
Built with love using:
egui/eframe — Immediate mode GUI for Rust
serde — Serialization framework
chrono — Date and time library
rfd — Native file dialogs
csv — CSV parsing and writing
dirs — Platform-specific directories
Special thanks to the Rust community for creating such an amazing ecosystem.
 Contact & Support
 Report a bug: Open an issue
 Request a feature: Open an issue
 Questions: Start a discussion
# Why GGUFF?
In a world of cloud-based apps that monetize your data, GGUFF Weight Editor takes a different approach:
Your data is yours — stored locally, never uploaded
No subscriptions — free forever, open source
No ads, no tracking — just a clean, focused tool
Community-driven — built by and for people who value privacy
Built with ❤️ in Tijuana as part of the OpenΦ initiative — creating useful tools that empower communities through open-source software.
<div align="center">

If you find this project useful, consider giving it a ⭐ on GitHub!
Star this repo · Report a bug · Request a feature
</div>
