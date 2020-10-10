use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我今天午餐吃了沙拉");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("我今天午餐吃了沙拉", post.content());
}
