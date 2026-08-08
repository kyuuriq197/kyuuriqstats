## Warning

Stable releases / Pre-releases and beta versions are published only on GitHub. 

# kyuuriqstats

A simple system information fetcher written in Rust.

## Features

- OS
- Kernel
- CPU
- RAM
- GPU
- Display
- Window manager
- Shell
- Terminal
- Packages
- Uptime
- Disks

## Start 

Add config.fish kyuuriqstats/fetch to the end

## Requirements

- Linux
- Rust
- pacman (for package count)
- niri (for display information)
- nvidia-smi (for NVIDIA GPU detection)

## Build

cargo build --release

## Run

cargo run --release
