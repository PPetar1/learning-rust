fn main() {
}

// Traits

pub trait Summary {// Here we define a trait Summary; any type implementing this trait must have
                   // functions defined with the same signature as those declared here
                   // pub means it can be used in other crates importing this one
    fn summarize(&self) -> String;
    fn default(&self) -> String {
        String::from("Default implementation of this function is used")// We can set a default
                                                                       // implementation of a
                                                                       // method like this and if
                                                                       // types that impl Summary
                                                                       // do not implement this
                                                                       // method as well, the
                                                                       // default will be used;
                                                                       // Also in this method scope
                                                                       // you can use other methods
                                                                       // from the trait since it
                                                                       // is guaranteed that they
                                                                       // will be implemented
    }
}
// If users from another crate wants to use the methods from Summary he needs to bring into scope
// both the types and the trait

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,    
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// One restriction with traits and external crates is that you cannot implement external trait on
// external crates (so for example you can't implement Display on Vec<T>)

pub fn notify(item: &impl Summary) {// This syntax means that we accept any type that implements
                                    // Summary trait; You can only use the methods from the trait
                                    // and nothing else because we don't know what the actual type
                                    // is
    println!("News! {}", item.summarize());
}
// The syntax above is just syntax sugar for:

pub fn notify_<T: Summary>(item: &T) {
    println!("News! {}", item.summarize());
}

// Multiple trait bounds

//pub fn notify(item: &(impl Summary + Display)) {}
//pub fn notify<T: Summary + Display>(item: &T) {}

fn some_function<T: Summary + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {3}
fn some_function_<T, U>(t: &T, u: &U) -> i32 
where// We can use where to make things look a bit nicer
    T: Summary + Clone, 
    U: Clone + Summary ,
{3}

fn returns_summarizable() -> impl Summary {// This can only be used if the function can return only
                                           // one type (so if something {Type1} else {Type2} wont
                                           // work). Not quite sure why is it useful then, but we
                                           // shall see
    Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {// Every Pair<T> has these methods
    fn new(x: T, y: T) -> Self {// Self here is Pair<T>
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {// Here we implement these methods only for types that have
                                       // Display and PartialOrd traits
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub trait ToString_ {
}

impl<T: Display> ToString_ for T {// These are called blanket implementation and they are
                                  // implemented only for types that also implement another trait

}
