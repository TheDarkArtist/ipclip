# Rust IP Address to Clipboard

This Rust program retrieves the local IP address and copies it to the clipboard on Linux and Windows systems.

## Table of Contents

- [About](#about)
- [Features](#features)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Usage](#usage)
- [Contributing](#contributing)

## About

This program uses Rust and shell commands to fetch the local IP address and copy it to the clipboard using platform-specific utilities (`xclip` on Linux and `clip.exe` on Windows).

## Features

- Retrieves the local IP address dynamically.
- Copies the IP address to the clipboard for easy access.

## Requirements

### Linux

- Linux operating system (tested on Ubuntu, Debian, Fedora, and Arch Linux).
- Rust programming language and Cargo build system.
- `xclip` utility installed (`sudo apt-get install xclip` on Debian-based systems).

### Windows

- Windows operating system.
- Rust programming language and Cargo build system.
- `clip.exe` utility available by default in Windows.

## Getting Started

Follow these instructions to set up and use the project on your Linux or Windows system.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/IAmKushagraSharma/ipclip.git
   cd ipclip

2. Ensure Rust is installed. If not, install it using:

    ```bash
    # Linux
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Windows
    Visit <https://www.rust-lang.org/tools/install> and follow the instructions.

    ```

3. Install xclip if not already installed:

    - Linux(xclip):

    ```bash
    sudo apt-get install xclip   # Debian-based
    sudo dnf install xclip       # Fedora
    sudo pacman -S xclip         # Arch Linux
    ```

    - Windows (clip.exe):
    `clip.exe` is available by default in Windows and does not require additional installation.

4. Build the project:

    ```bash
    # Linux
    cargo build --release

    # Windows
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

### Usage

Run the compiled executable to copy the local IP address to the clipboard:

```bash
# Linux
./target/release/ipclip

# Windows
.\target\release\ipclip.ext
```

### Contributing

Contributions are welcome! Please fork the repository and submit pull requests. For major changes, open an issue first to discuss potential updates.
