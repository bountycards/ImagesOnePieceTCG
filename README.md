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

## Docker Usage

You can run this application in Docker, which will automatically update and commit changes to WebP images every 6 hours.

### Prerequisites for Docker

- Docker installed
- GitHub Personal Access Token (needed for pushing changes)

### Running with Docker

1. Set your GitHub Personal Access Token (optional, but required for pushing changes):

   ```bash
   export GITHUB_TOKEN=your_personal_access_token
   ```

2. Run the application using the provided script:
   ```bash
   chmod +x run_docker.sh
   ./run_docker.sh
   ```

This will:

- Build a Docker image for the application
- Run a container that executes the application every 6 hours
- Automatically commit changes to the webp_images folder
- Push changes to GitHub if a token is provided

### Manual Docker Commands

If you prefer to run Docker commands manually:

```bash
# Build the image
docker build -t one-piece-image-converter .

# Run the container
docker run -d \
  --name one-piece-converter \
  -e GITHUB_TOKEN=your_personal_access_token \
  -e GITHUB_REPOSITORY=username/repo-name \
  -v $(pwd):/app \
  one-piece-image-converter
```

### Viewing Logs

```bash
docker logs one-piece-converter
```
