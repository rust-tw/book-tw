// ANCHOR: all
use blog::Post;

// ANCHOR: here
fn main() {
    let mut post = Post::new();

    post.add_text("我今天午餐吃了沙拉");
    assert_eq!("", post.content());
    // ANCHOR_END: here

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("我今天午餐吃了沙拉", post.content());
    // ANCHOR: here
}
// ANCHOR_END: here
// ANCHOR_END: all
