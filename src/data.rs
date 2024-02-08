use chrono::{DateTime, Utc};
use clap::Parser;
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Post {
    pub id: i32,
    pub likes: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub category: String,
    pub user: User,
}

impl Post {
    pub fn display(&self) {
        println!("{}: {}", "TITLE".bright_red().bold(), self.title);
        println!("{}: {}", "USER".bright_green().bold(), self.user.username);
        println!("{}: {}", "URL".bright_yellow().bold(), self.get_url());
    }
}

impl Post {
    pub fn get_url(&self) -> String {
        return format!("https://beta.deeeep.io/forum/en/{}", self.id);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The searched for title
    pub search: Option<String>,

    /// Filters results by username.
    #[arg(short, long)]
    pub user: Option<String>,

    /// Filters results by user id.
    #[arg(short, long)]
    pub id: Option<u64>,

    /// Filters results by category.
    #[arg(long)]
    pub category: Option<String>,

    /// Sets a custom search length in seconds. Can be specified as a decimal, will be rounded to the nearest millisecond.
    #[arg(short, long, default_value_t = 30.0)]
    pub timeout: f64,

    /// Number of posts to parse.
    #[arg(short, long, default_value_t = 16000)]
    pub count: u64,

    /// Number of posts to display.
    #[arg(short, long, default_value_t = 10)]
    pub display: usize,
}

impl Args {
    pub fn filter(&self, post: &Post) -> bool {
        if self.user.is_some() && &post.user.username != self.user.as_ref().unwrap() {
            return false;
        }
        if self.id.is_some() && post.user.id != self.id.unwrap() {
            return false;
        }

        if self.category.is_some() && &post.category != self.category.as_ref().unwrap() {
            return false;
        }

        return true;
    }

    pub fn timeout_as_millis(&self) -> u64 {
        return (self.timeout * 1000.0).round() as u64;
    }
}
