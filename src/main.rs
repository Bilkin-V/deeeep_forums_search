use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use data::Post;
use reqwest::Client;
use tokio::time;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

use crate::data::Args;

mod data;
mod req;

fn display(posts: &Vec<Post>) {
    for post in posts {
        post.display();
        println!();
        println!();
    }
}

fn init_tracing() {
    let layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .with_regex(false)
                .from_env()
                .expect("Couldn't construct tracing_subscriber env_filter"),
        );

    tracing_subscriber::registry().with(layer).init();
}

#[tokio::main]

async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    color_backtrace::install();
    init_tracing();

    let args = Args::parse();

    let posts = match &args.search {
        Some(search) => time::timeout(
            Duration::from_millis(args.timeout_as_millis()),
            req::search(&search, &args, &Client::new()),
        )
        .await
        .unwrap()
        .unwrap(),
        None => time::timeout(
            Duration::from_millis(args.timeout_as_millis()),
            req::accumulate(&args, &Client::new()),
        )
        .await
        .unwrap()
        .unwrap(),
    };

    display(&posts);

    return Ok(()); // #[tokio::main] macro breaks syntax highlighting here.
}
