# devlog

A simple CLI tool to log your daily development progress.

## Features
- Add logs with tags
- Filter logs by tags
- Persistent storage (JSON)

## Installation

### Local
cargo install --path .

### From source
git clone ...
cd devlog-cli
cargo build --release

## Usage

devlog add "learned Rust ownership" --tag rust
devlog list
devlog list --tag rust
devlog help