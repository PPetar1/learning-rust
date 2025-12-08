/// # Documentation comments
/// These are documentation comments
/// they can use markdown and should explain
/// what the code does

//! #Crate info
//! These comments relate to the item in which they are written
//! not to the item after it, so here they relate to the crate as a whole

fn main() {
    println!("Hello, world!");
}
// pub use self::module::another_module::something_useful// This is how you can reexport something
// from some module to be seen on this level; It can be useful to expose some things a bit more
// nicely for some external user of the crate 
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
