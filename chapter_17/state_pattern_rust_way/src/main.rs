use state_pattern_rust_way::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Something");

    let post = post.request_review();
   
    let post = post.approve();

    assert_eq!("Something", post.content());
}
