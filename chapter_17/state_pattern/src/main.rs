// We will implement a blog post unsing a obj oriented design pattern - state pattern
// We define a set of states a value can have internally, which are represented by state objects
// and the behavior of the value changes based on its state

// For our example we will use a blog post
// It will start with an empty draft
// When the draft is done a review is requested
// When the post is approved it will get published
use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Something");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Something", post.content());
}
