// For our example we will use a blog post
// It will start with an empty draft
// When the draft is done a review is requested
// When the post is approved it will get published


pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)// as_ref takes an Option<T> and returns
                                                  // Option<&T>, we do this because we dont want to
                                                  // take ownership of value in state
                                                  // We pass self to the content function that will
                                                  // be different for each of the states (Draft and
                                                  // PendingReview will use default implementation
                                                  // returning "" and Published will return the
                                                  // content of the self Post we passed as the
                                                  // argument)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
        //self.state = self.state.request_review();// If we removed the option and used this - it
        //woudln't work. Thats because when you call self.state.request_review() you are moving the
        //state ownership to the method and leaving the Post in an invalid state (where it doesn't
        //have anything in the state field). Rust compiler does not allow this just because we are
        //putting the value back into state straight away. Option::take() is used instead and this
        //takes ownership of the value in Option while leaving None in its place
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State { 
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Draft>) -> Box<dyn State> {// The self: Box<Self> syntax means that
                                                           // the method is only valid when called
                                                           // on a box holding this type
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
