use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
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

pub fn collate_values(args: Cli, config: &Config) -> (u8, u8, String) {
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
            .clone()
            .expect("Error getting selected list from flag")
    } else if config.selected_list.is_some() {
        config
            .selected_list
            .clone()
            .expect("Need to specify a selected list")
    } else {
        panic!("Need to set selected list")
    };

    (count, skip_amount, list)
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
