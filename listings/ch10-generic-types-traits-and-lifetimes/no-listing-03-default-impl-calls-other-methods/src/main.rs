use chapter10::{self, Summary, Tweet};

fn main() {
    // ANCHOR: here
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 則新推文：{}", tweet.summarize());
    // ANCHOR_END: here
}
