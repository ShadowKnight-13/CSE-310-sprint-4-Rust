# Rust D&D Dice Roller (egui/eframe)

Cross-platform (Windows + Linux) D&D dice roller built with Rust.

## Project layout

- `dice_core`: library crate with parser, roller, types, and tests
- `dice_gui`: binary crate with `egui/eframe` GUI and CLI fallback

## Features mapped to course requirements

- Immutable + mutable variables: parser and GUI state use both `let` and `let mut`
- Loops: rolling multiple dice and iterating roll history
- Functions with borrowing and ownership:
  - borrowing: `make_roll(&str)`
  - ownership transfer: `make_roll_owned(String)`
- `Vec`: stores individual die results
- `match`: used for parser token handling and GUI error handling
- Stretch challenge:
  - `struct Roll`
  - parser returns `Result<_, ParseError>`

## Supported notation examples

- `2d6+1`
- `d20`
- `4d8-2`

## 1) Prerequisite checks and installation

### Windows (PowerShell)

Check if Rust tools exist:

```powershell
rustc --version
cargo --version
rustup --version
```

If missing, install Rustup:

```powershell
winget install --id Rustlang.Rustup -e
```

Then install stable toolchain:

```powershell
rustup toolchain install stable
rustup default stable
```

Automated helper script:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\bootstrap_windows.ps1
```

### Linux (bash)

Check if Rust tools exist:

```bash
rustc --version
cargo --version
rustup --version
```

If missing, install rustup + stable:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
source "$HOME/.cargo/env"
```

Run helper script:

```bash
bash ./scripts/bootstrap_linux.sh
```

If native build packages are missing, run one of these manually with sudo:

```bash
sudo apt update && sudo apt install -y build-essential pkg-config libx11-dev libxi-dev libgl1-mesa-dev
# OR
sudo dnf install -y gcc gcc-c++ make pkgconfig libX11-devel libXi-devel mesa-libGL-devel
```

## 2) Build and run

From the workspace root:

```bash
cargo test -p dice_core
cargo run -p dice_gui
```

CLI fallback (same core logic):

```bash
cargo run -p dice_gui -- 2d6+1
```

## 3) Release packaging (single native binary)

Build release binary:

```bash
cargo build --release -p dice_gui
```

Output locations:

- Windows EXE: `target/release/dice_gui.exe`
- Linux ELF: `target/release/dice_gui`

Optional installer tooling:

- Debian package: `cargo install cargo-deb` then `cargo deb`
- Windows MSI/installer options: WiX Toolset, Inno Setup, or NSIS wrapping `target/release/dice_gui.exe`

## 4) Workspace creation commands (for reference)

```bash
cargo new --lib dice_core
cargo new --bin dice_gui
```

Then create workspace `Cargo.toml`:

```toml
[workspace]
members = ["dice_core", "dice_gui"]
resolver = "2"
```


# Project Title (Update)

Add a description of your project here.

## Instructions for Build and Use

Steps to build and/or run the software:

1. First step here
2.
3.

Instructions for using the software:

1. First step here
2.
3.

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* First thing here
*
*

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Website Title](Link)
*
*

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] First thing here
* [ ]
* [ ]