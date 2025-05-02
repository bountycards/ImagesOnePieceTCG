# One Piece Card Image Converter (Rust Version)

A Rust implementation of the One Piece Card Image Converter that downloads card images and converts them to WebP format.

## Prerequisites

- Rust and Cargo installed
- The `one-piece-card-game-json` data package in the node_modules directory

## Dependency Note

This project uses several dependencies that may need version adjustments based on your Rust version:

- If you encounter errors like "package X requires rustc 1.81", you may need to:
  - Upgrade your Rust version, or
  - Downgrade specific dependencies with: `cargo update <package>@<current-ver> --precise <compatible-ver>`

## Installation

```bash
git clone <repository-url>
cd one-piece-image-converter/rust-converter
cargo build --release
```

## Usage

```bash
# Process all languages (en, jp)
cargo run --release

# Process specific languages
cargo run --release -- --languages en,jp

# Customize settings
cargo run --release -- --concurrency-limit 10 --webp-quality 95 --regular-delay 100 --extended-delay 3000
```

## Command-line Options

- `--concurrency-limit`, `-c`: Number of concurrent downloads (default: 5)
- `--regular-delay`, `-r`: Delay between downloads in milliseconds (default: 200)
- `--extended-delay`, `-e`: Extended delay after every 20 images in milliseconds (default: 1000)
- `--webp-quality`, `-w`: WebP quality setting 1-100 (default: 80)
- `--languages`, `-l`: Languages to process, comma-separated (default: en,jp)
- `--base-dir`, `-b`: Output directory for WebP images (default: webp_images)

## Features

- Concurrent image downloading with rate limiting
- WebP conversion with quality settings
- Progress bar visualization
- Command-line configuration options
- Detailed error handling and logging
