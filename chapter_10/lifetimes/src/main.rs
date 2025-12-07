fn main() {
    let string1 = "abcd";
    let string2 = "xyz";

    let result = longest(string1, string2);
    println!("{result}");
}
// Lifetimes are used to specify to the compiler the scope in which references must be valid and
// we need to specify them when working with references


// This function wouldnt compile because the compiler doesn't know what will be the lifetime of the
// return reference
// fn longest(x: &str, y: &str) -> &str {
//      if x.len() > y.len() {
//          x
//      } else {
//          y
//      }
//  }

// This uses lifetimes and works
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {// Here we are saying that we will use some
                                                   // lifetime 'a and that x, y and the slice
                                                   // returned from the function will live at least
                                                   // as long as lifetime 'a. In practice here 'a
                                                   // will be the shortest of the two lifetimes (x
                                                   // and y) and we are saying that this reference
                                                   // we are returning will be valid for at least
                                                   // as long as the lifetime of the variable that
                                                   // will go out of scope first
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Using lifetime with structs

struct ImportantExcerpt<'a> {// Here we are saying that the instance of this struct cannot outlive the
                    // reference in the part field
    part: &'a str,
}

// Lifetime elision - in some simple cases you can omit the lifetime annotations and the compiler
// will infer those by itself

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// There are 3 rules the compiler uses to perform lifetime elision
// The first rule is that the compiler assigns a lifetime parameter to
// each parameter that’s a reference. In other words, a function with
//one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
//a function with two parameters gets two separate lifetime
//parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
//The second rule is that, if there is exactly one input lifetime
//parameter, that lifetime is assigned to all output lifetime parameters:
//fn foo<'a>(x: &'a i32) -> &'a i32.
//The third rule is that, if there are multiple input lifetime parameters,
//but one of them is &self or &mut self because this is a method, the
//lifetime of self is assigned to all output lifetime parameters. This
//third rule makes methods much nicer to read and write because
//fewer symbols are necessary.

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {// Here the third rule is
                                                                    // applied but i am not quite
                                                                    // sure why does the third rule
                                                                    // make sense
        println!("Attention please: {announcement}");
        self.part
    }
}

// Static lifetime

// let s: &'static str = "Static lifetime"; // String literals have a static lifetime because they
                                            // are stored in the code but here we esplicitly mark
                                            // them with 'static; we can use 'static on other
                                            // variables to make their lifetime be the whole
                                            // program execution

// Syntax for specifying generic types, trait bounds and lifetimes together


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetimes are a bit weird but not too difficult to use i think - i think the intended way to use
// them is to forget about them and then when the compiler shows issues go back and see what did
// you do wrong
