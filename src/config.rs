use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;
use xdg::BaseDirectories;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub count: Option<u8>,
    #[arg(short = 'l', long)]
    pub selected_list: Option<String>,
    #[arg(short, long)]
    pub skip_amount: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub count: Option<u8>,
    pub skip_amount: Option<u8>,
    pub selected_list: Option<String>,
    pub lists: HashMap<String, Vec<String>>,
}

pub fn parse_cli() -> Cli {
    let args = Cli::parse();
    args
}

pub fn validate_config() -> Config {
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
