use chrono::{DateTime, Utc};
use clap::Parser;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::get;
use rss::Channel;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use terminal_link::Link;
use tokio;
use toml;
use xdg::BaseDirectories;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    count: Option<u8>,
    #[arg(short = 'l', long)]
    selected_list: Option<String>,
    #[arg(short, long)]
    skip_amount: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct Config {
    count: Option<u8>,
    skip_amount: Option<u8>,
    selected_list: Option<String>,
    lists: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct FeedItem {
    title: String,
    description: String,
    link: String,
    pub_date: DateTime<Utc>,
}

async fn fetch_rss(url: &str, pb: &ProgressBar) -> Result<Channel, Box<dyn Error>> {
    let response = get(url).await?.text().await?;
    let channel = Channel::read_from(response.as_bytes())?;
    pb.inc(1);
    pb.set_message(format!("Processing: {}", channel.title));
    Ok(channel)
}

fn parse_feed(channel: &Channel) -> Vec<FeedItem> {
    channel
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap_or("No title").to_string(),
            description: item.description().unwrap_or("No description").to_string(),
            link: item.link().unwrap_or("No link").to_string(),
            pub_date: item
                .pub_date()
                .and_then(|date_str| DateTime::parse_from_rfc2822(date_str).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|| Utc::now()),
        })
        .collect()
}

fn validate_config() -> Config {
    let xdg_dirs = BaseDirectories::new().expect("Failed to get XDG directories");
    let config_path = xdg_dirs
        .place_config_file("packard/config.toml")
        .expect("Failed to determine config file path");

    if !config_path.exists() {
        eprintln!("Configuration file not found at {:?}", config_path);
    }

    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Config = toml::de::from_str(&config_content).expect("Failed to parse TOML");
    config
}

fn trim_chars(input: &str) -> String {
    let trimmed: String = input.chars().take(256).collect();

    if trimmed.len() < input.len() {
        format!("{}...", trimmed)
    } else {
        trimmed
    }
}

async fn run_tasks(sources: Vec<String>, count: u8, skip: u8, pb: &ProgressBar) -> Vec<FeedItem> {
    let fetch_futures: Vec<_> = sources.iter().map(|url| fetch_rss(url, &pb)).collect();

    let channels = join_all(fetch_futures).await;

    let mut all_items = Vec::new();

    for channel in channels.into_iter().filter_map(Result::ok) {
        let feed_items = parse_feed(&channel);
        all_items.extend(feed_items);
    }

    all_items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
    all_items.truncate((count + skip).into());
    let removed_items = all_items.split_off(skip.into());

    removed_items
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = validate_config();
    let args = Cli::parse();

    if args.verbose {
        println!("{:?}", args);
        println!("Selected list: {:?}", config.selected_list);
        println!("Items: {:?}", config.lists);
    }

    let count: u8 = if args.count.is_some() {
        args.count.expect("Count flag wrong?")
    } else if config.count.is_some() {
        config.count.expect("Unable to use count value from config")
    } else {
        8
    };

    let skip_amount: u8 = if args.skip_amount.is_some() {
        args.skip_amount.expect("Skip amount flag wrong?")
    } else if config.skip_amount.is_some() {
        config
            .skip_amount
            .expect("Unable to use skip amount value from config")
    } else {
        0
    };

    let list: String = if args.selected_list.is_some() {
        args.selected_list
            .expect("Error getting selected list from flag")
    } else if config.selected_list.is_some() {
        config
            .selected_list
            .expect("Need to specify a selected list")
    } else {
        panic!("Need to set selected list")
    };

    if let Some(values) = config.lists.get(&list) {
        let pb = indicatif::ProgressBar::new(12);
        pb.set_style(
            ProgressStyle::with_template("[{elapsed}] {bar:40.green/black} {msg}").unwrap(),
        );

        let all_items = run_tasks(values.to_vec(), count, skip_amount, &pb).await;
        pb.finish_and_clear();

        for item in all_items {
            println!(
                "\x1b[1m>\x1b[0m \x1b[1;32m{}\x1b[0m\n\x1b[3m\x1b[2m{}\x1b[0m\n\x1b[2m{}\x1b[0m\n",
                Link::new(&item.title, &item.link),
                trim_chars(&item.description),
                item.pub_date.to_string()
            );
        }
    } else {
        panic!("Have you specified your site lists and chosen one?");
    }

    Ok(())
}
