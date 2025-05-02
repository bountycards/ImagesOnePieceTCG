use anyhow::{Context, Result};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// Simple command line parsing without the clap derive macro
#[derive(Clone)]
struct Config {
    concurrency_limit: usize,
    regular_delay: u64,
    extended_delay: u64,
    webp_quality: u8,
    languages: Vec<String>,
    base_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            concurrency_limit: 5,
            regular_delay: 200,
            extended_delay: 1000,
            webp_quality: 80,
            languages: vec!["en".to_string(), "jp".to_string()],
            base_dir: PathBuf::from("webp_images"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Card {
    card_name: String,
    image_url: String,
    image_name: String,
}

struct Stats {
    download_count: usize,
    total_skipped: usize,
}

async fn load_cards(lang: &str) -> Result<Vec<Card>> {
    let file_path = PathBuf::from("node_modules")
        .join("one-piece-card-game-json")
        .join(lang)
        .join("cards.json");

    let data = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read {} card data", lang))?;

    let cards: Vec<Card> = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse {} card data as JSON", lang))?;

    Ok(cards)
}

fn create_output_directories(config: &Config) -> Result<()> {
    for lang in &config.languages {
        let dir = config.base_dir.join(lang);
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create directory: {:?}", dir))?;
    }
    Ok(())
}

async fn download_image(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to download image from {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download: {} {}",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("Unknown")
        );
    }

    let bytes = response
        .bytes()
        .await
        .with_context(|| "Failed to read image bytes")?;

    Ok(bytes.to_vec())
}

fn convert_to_webp(image_buffer: &[u8], output_path: &Path, quality: u8) -> Result<()> {
    let img = image::load_from_memory(image_buffer)
        .with_context(|| "Failed to load image from memory")?;

    let encoder = webp::Encoder::from_image(&img)
        .map_err(|e| anyhow::anyhow!("Failed to create WebP encoder: {}", e))?;

    let webp = encoder.encode(quality as f32);
    fs::write(output_path, &*webp)
        .with_context(|| format!("Failed to write WebP image to {:?}", output_path))?;

    Ok(())
}

async fn process_card(
    card: &Card,
    lang: &str,
    config: &Config,
    stats: Arc<Mutex<Stats>>,
) -> Result<bool> {
    let output_path = config
        .base_dir
        .join(lang)
        .join(format!("{}.webp", card.image_name));

    if output_path.exists() {
        let mut stats = stats.lock().unwrap();
        stats.total_skipped += 1;
        return Ok(false);
    }

    println!("Processing: {} ({})", card.card_name, lang);

    let image_buffer = download_image(&card.image_url).await?;

    convert_to_webp(&image_buffer, &output_path, config.webp_quality)?;

    println!("Successfully processed: {} ({})", card.image_name, lang);

    sleep(Duration::from_millis(config.regular_delay)).await;

    Ok(true)
}

async fn process_language(lang: &str, config: &Config, stats: Arc<Mutex<Stats>>) -> Result<()> {
    let cards = load_cards(lang).await?;
    println!("Processing {} cards for language: {}", cards.len(), lang);

    let pb = ProgressBar::new(cards.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let tasks = futures::stream::iter(cards.iter().map(|card| {
        let lang = lang.to_string();
        let config = config.clone();
        let stats = Arc::clone(&stats);
        let pb = pb.clone();

        async move {
            let result = process_card(card, &lang, &config, stats.clone()).await;

            if let Ok(true) = result {
                let should_sleep;
                let download_count;
                {
                    let mut stats = stats.lock().unwrap();
                    stats.download_count += 1;
                    download_count = stats.download_count;
                    should_sleep = stats.download_count % 20 == 0;
                }

                if should_sleep {
                    println!(
                        "Downloaded {} images. Taking a longer break...",
                        download_count
                    );
                    sleep(Duration::from_millis(config.extended_delay)).await;
                }
            }

            pb.inc(1);
            result
        }
    }))
    .buffer_unordered(config.concurrency_limit)
    .collect::<Vec<_>>();

    tasks.await;
    pb.finish_with_message(format!("Completed {} cards", lang));

    Ok(())
}

fn parse_args() -> Config {
    let mut config = Config::default();

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--concurrency-limit" | "-c" => {
                if i + 1 < args.len() {
                    if let Ok(value) = args[i + 1].parse() {
                        config.concurrency_limit = value;
                    }
                    i += 1;
                }
            }
            "--regular-delay" | "-r" => {
                if i + 1 < args.len() {
                    if let Ok(value) = args[i + 1].parse() {
                        config.regular_delay = value;
                    }
                    i += 1;
                }
            }
            "--extended-delay" | "-e" => {
                if i + 1 < args.len() {
                    if let Ok(value) = args[i + 1].parse() {
                        config.extended_delay = value;
                    }
                    i += 1;
                }
            }
            "--webp-quality" | "-w" => {
                if i + 1 < args.len() {
                    if let Ok(value) = args[i + 1].parse() {
                        config.webp_quality = value;
                    }
                    i += 1;
                }
            }
            "--languages" | "-l" => {
                if i + 1 < args.len() {
                    config.languages = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    i += 1;
                }
            }
            "--base-dir" | "-b" => {
                if i + 1 < args.len() {
                    config.base_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    config
}

// Use tokio's macro for async main
#[tokio::main]
async fn main() -> Result<()> {
    let config = parse_args();

    println!("=== One Piece Card Image Converter ===");
    println!(
        "Concurrency: {}, Quality: {}%",
        config.concurrency_limit, config.webp_quality
    );

    create_output_directories(&config)?;

    let target_langs = if std::env::args().len() > 1 {
        config.languages.clone()
    } else {
        config.languages.clone()
    };

    if target_langs.is_empty() {
        anyhow::bail!(
            "No valid languages specified. Available options: {}",
            config.languages.join(", ")
        );
    }

    println!("Processing languages: {}", target_langs.join(", "));

    let stats = Arc::new(Mutex::new(Stats {
        download_count: 0,
        total_skipped: 0,
    }));

    for lang in target_langs {
        process_language(&lang, &config, Arc::clone(&stats)).await?;
    }

    let stats = stats.lock().unwrap();
    println!("\n=== Summary ===");
    println!("Downloaded: {} new images", stats.download_count);
    println!("Skipped: {} existing images", stats.total_skipped);
    println!("Conversion complete!");

    Ok(())
}
