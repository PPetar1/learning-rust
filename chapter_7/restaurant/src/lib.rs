// The main division in rust is a package that is made from multiple crates that can have multiple
// modules

mod front_of_house {// This doesn't need to be private in our example because front_of_house and
                    // eat_at_restaurant are defined in the same module (they are siblings)
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,// public
        seasonal_fruit: String,// private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();//super represents the parent module, in this case crate
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit that comes
    // with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}

use crate::front_of_house::hosting;// Idiomatic is to bring the parent module into scope
use std::collections::HashMap;// Except for structs and enums (this is just a convention)
use std::fmt;
use std::io;

fn f1(a: fmt::Result) {}

fn f2(a: io::Result<()>) {}// Here we have to make an exception since both fmt and io modules
                           // have the Result type and you cant have multiple same named types in
                           // a single scope

use std::fmt::Result;
use std::io::Result as IoResult;// We can also do this, now we can reference io::Result with
                                // IoResult

mod another_module {
    pub fn use_is_not_valid_here() {
        // hosting::add_to_waitlist();// This woudln't compile, use is only for the scope in witch
                                      // it is declared
        super::hosting::add_to_waitlist();// This does compile
    }
}

//pub use crate::front_of_house::hosting;// This would reexport the hosting module and it would 
                                         //look
                                       // to external users of this crate like its defined on this
                                       // level; they would be able to access the function with
                                       // restaurant::hosting::add_to_waitlist();

pub fn eat_at_restaurant1() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    //Because we used use we can do this as well
    hosting::add_to_waitlist();
}

use rand::Rng;// We can call on external dependency declared in Cargo.toml file like this, by using
              // its name as the root

use std::{cmp::Ordering, io};// You can use nested paths to include multiple paths at once
                             // use std::io{self, Write}; would include std::io and std::io::Write

use std::collections::*;// This brings all public items from the path into scope and is called the
                        // glob operator

// If you just declare the module mod module; the compiler will look for the code in
// src/module.rs and src/module/mod.rs; for a submodule of module name submodule the compiler will
// look into src/module/submodule.rs or src/module/submodule/mod.rs




pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
