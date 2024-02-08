use std::collections::BinaryHeap;

use crate::data::{Args, Post};
use anyhow::Result;
use reqwest::Client;

pub async fn accumulate(args: &Args, client: &Client) -> Result<Vec<Post>> {
    let posts: Vec<Post> = serde_json::from_str(
        client
            .get(format!(
                "https://apibeta.deeeep.io/forumPosts/en?count={}&page=1&order=hot&period=always",
                args.count
            ))
            .send()
            .await?
            .text()
            .await?
            .as_str(),
    )?;

    let mut filtered = Vec::<Post>::with_capacity(args.display);

    for post in posts {
        if args.filter(&post) {
            filtered.push(post);

            if filtered.len() == args.display {
                break;
            }
        }
    }

    return Ok(filtered);
}

pub async fn search(search: &str, args: &Args, client: &Client) -> Result<Vec<Post>> {
    let posts: Vec<Post> = serde_json::from_str(
        client
            .get(format!(
                "https://apibeta.deeeep.io/forumPosts/en?count={}&page=1&order=hot&period=always",
                args.count
            ))
            .send()
            .await?
            .text()
            .await?
            .as_str(),
    )?;

    let mut heap = BinaryHeap::<(isize, Post)>::with_capacity(10);

    for post in posts {
        let best_match = sublime_fuzzy::best_match(search, &post.title);

        match best_match {
            Some(s) => {
                if !args.filter(&post) {
                    continue;
                }

                if heap.is_empty() {
                    heap.push((-s.score(), post));
                    continue;
                }

                if heap.peek().unwrap().0 <= -s.score() {
                    continue;
                }

                if heap.len() == 10 {
                    heap.pop();
                }
                heap.push((-s.score(), post));
            }
            None => continue,
        }
    }
    let mut heap = heap.into_vec();
    heap.sort();

    return Ok(heap
        .into_iter()
        .map(|(_, post)| post)
        .collect::<Vec<Post>>());
}
