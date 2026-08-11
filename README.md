# System_Overview

This is a terminal-tool, designed to monitor and adjust your computer from the terminal. It is meant to be used on
machines with the role of server, but can also be used on personal devices for development.

```
 ____            _
/ ___| _   _ ___| |_ ___ _ __ ___
\___ \| | | / __| __/ _ \ '_ ` _ \
 ___) | |_| \__ \ ||  __/ | | | | |
|____/ \__, |___/\__\___|_| |_| |_|
 / _ \_|___/____ _ ____   _(_) _____      __
| | | \ \ / / _ \ '__\ \ / / |/ _ \ \ /\ / /
| |_| |\ V /  __/ |   \ V /| |  __/\ V  V /
 \___/  \_/ \___|_|    \_/ |_|\___| \_/\_/
```

## Status

This Program is not finished yet, "Resources" works already for the most part, but "Containers" and "Network" aren't implemented yet.

## Features

The TUI has three tabs, "Resources", "Containers" and "Network".

Under Resources, you have your typical monitoring data, f. e. Cpu Graphs, Disk Utilization, etc.

Under Containers, you have your option to monitor and configure your containers, and everything related to them.

Under Network, you can configure your network settings, f. e. VPN, SSH, etc.

## Requirements

- Rust (stable toolchain)
- Linux or Windows
- NVIDIA GPU + drivers for GPU metrics (optional, you just might not see any GPU data)

## Build & Run

```bash
git clone https://github.com/laurens-hertzer/system_overview
cd system_overview
cargo run --release
```

## Keybindings

| Key         | Action                               |
|-------------|--------------------------------------|
| `q` / `Esc` | Quit                                 |
| `h` / `←`   | Previous tab                         |
| `l` / `→`   | Next tab                             |
| `c`         | Toggle chart colour (green / yellow) |

## Project Structure

```
src/
├── main.rs          # Entry point, spawns background threads
├── backend.rs       # Event types and data-collection threads (CPU, RAM, GPU, Disk)
├── tui.rs           # App state and ratatui rendering
├── utils.rs         # Helper functions (unit conversions, logo rendering)
└── cliArgPars.rs    # CLI argument definitions (clap)
```

## Dependencies

| Crate          | Purpose                              |
|----------------|--------------------------------------|
| `ratatui`      | Terminal UI rendering                |
| `crossterm`    | Cross-platform terminal input/output |
| `sysinfo`      | CPU, RAM, and disk metrics           |
| `nvml-wrapper` | NVIDIA GPU metrics via NVML          |
| `clap`         | CLI argument parsing                 |
| `color-eyre`   | Error handling                       |
