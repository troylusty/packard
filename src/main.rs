use std::error::Error;
use terminal_link::Link;
use tokio;

mod config;
mod data;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::validate_config();
    let args = config::parse_cli();
    let (count, skip_amount, list) = config::collate_values(args, &config);

    if let Some(values) = config.lists.get(&list) {
        let pb = indicatif::ProgressBar::new(
            values
                .len()
                .try_into()
                .expect("Could not convert list length"),
        );
        let all_items = data::run_tasks(values.to_vec(), count, skip_amount, &pb).await;
        pb.finish_and_clear();

        for item in all_items {
            println!(
                "\x1b[1m>\x1b[0m \x1b[1;32m{}\x1b[0m\n\x1b[3m\x1b[2m{}\x1b[0m\n\x1b[2m{}\x1b[0m\n",
                Link::new(&item.title, &item.link),
                utils::trim_chars(&item.description),
                item.pub_date.to_string()
            );
        }
    } else {
        panic!("Have you specified your site lists and chosen one?");
    }

    Ok(())
}
