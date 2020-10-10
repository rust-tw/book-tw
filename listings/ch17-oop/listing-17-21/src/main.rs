use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我今天午餐吃了沙拉");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("我今天午餐吃了沙拉", post.content());
}
