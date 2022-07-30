pub mod aggregator;

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Nestor"),
        content: String::from(
            "of course, this is my first text"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}