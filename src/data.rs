use chrono::{DateTime, Utc};
use futures::future::join_all;
use indicatif::ProgressBar;
use reqwest::get;
use rss::Channel;
use std::error::Error;

#[derive(Debug)]
pub struct FeedItem {
    pub title: String,
    pub description: String,
    pub link: String,
    pub pub_date: DateTime<Utc>,
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

pub async fn run_tasks(
    sources: Vec<String>,
    count: u8,
    skip: u8,
    pb: &ProgressBar,
) -> Vec<FeedItem> {
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
