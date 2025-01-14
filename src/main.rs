use indicatif::ProgressStyle;
use std::error::Error;
use terminal_link::Link;
use tokio;

mod config;
mod data;

fn trim_chars(input: &str) -> String {
    let trimmed: String = input.chars().take(256).collect();

    if trimmed.len() < input.len() {
        format!("{}...", trimmed)
    } else {
        trimmed
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::validate_config();
    let args = config::parse_cli();

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

        let all_items = data::run_tasks(values.to_vec(), count, skip_amount, &pb).await;
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
