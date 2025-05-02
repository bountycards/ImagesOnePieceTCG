FROM rust:slim-bookworm

# Install git and cron
RUN apt-get update && apt-get install -y \
    git \
    cron \
    && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Copy the application code
COPY . .

# Configure git
RUN git config --global user.name "Docker Automation" && \
    git config --global user.email "docker@automation.com"

# Make sure the update script is executable
RUN chmod +x /app/update_webp_images.sh

# Setup cron job - run every 6 hours
RUN echo "0 */6 * * * /app/update_webp_images.sh >> /app/cron.log 2>&1" > /etc/cron.d/update-cron && \
    chmod 0644 /etc/cron.d/update-cron && \
    crontab /etc/cron.d/update-cron

# Build the Rust application
RUN cargo build --release

# Create a volume for persistency
VOLUME /app

# Start cron in the foreground
CMD ["cron", "-f"] 