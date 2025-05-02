#!/bin/bash

cd /app
cargo run --release

# Check if there are changes in webp_images folder
if [ -n "$(git status --porcelain webp_images)" ]; then
    echo "Changes detected in webp_images folder"
    git add webp_images
    git commit -m "Auto-update webp images"
    # Push changes if the GitHub token is configured
    if [ -n "$GITHUB_TOKEN" ]; then
        git push https://${GITHUB_TOKEN}@github.com/${GITHUB_REPOSITORY}.git $(git rev-parse --abbrev-ref HEAD)
    else
        echo "GITHUB_TOKEN not set. Skipping push."
    fi
else
    echo "No changes in webp_images folder"
fi 