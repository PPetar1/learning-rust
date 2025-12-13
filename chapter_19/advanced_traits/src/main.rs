// Traits with associated types

pub trait Iterator {
    type Item;// This is an associated type, similar to generic type but only one can be defined
              // per type implementing this trait. If we had a pub trait Iterator<T> then we could
              // have impl<u32> Iterator<u32> for MyType, impl<i32> Itera.....

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;// We fix the Items type here
    fn next(&mut self) -> Option<Self::Item> { Some(2) }
}

// You can overload the operators by implementing their traits from std::operators

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Add trait is defined in std lib like:

trait Add_<Rhs=Self> {// If Rhs is not specified the default will be used (in this case the type on
                      // which the trait is implemented, Point in the example below) 
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

// Identically named methods in different Traits

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Flying as a Pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Flying as a Wizard");
    }
}

impl Human {
    fn fly(&self) {
        println!("Trying to fly as Human");
    }
}

// Using Supertraits
// We define a trait that depends on another trait (supertrait). We require that every type that
// implements our trait firstly implement the supertrait. We can use supertraits methods in our
// trait

use std::fmt;

trait OutlinePrint: fmt::Display {// We require that every trait implementing OutlinePrint has
                                  // Display implemented

    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));

    }

}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Newtype Pattern
// We said before that we cannot implement a trait on a type if both are external to our trait
// It is possible to get around this by using this Pattern

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 });

    let person = Human;
    person.fly();// This will call the fly method implemented on human directly
    Pilot::fly(&person);// This will fly as the Pilot
    Wizard::fly(&person);// This will fly as the Wizard
    <Human as Wizard>::fly(&person);// This is a fully qualified syntax which we might need to use
                                    // in cases where the method doesn't take any parameters (in
                                    // that case multiple types can implement the Wizard trait so
                                    // we need to distinguish between them)
                                    // Syntax is <Type as Trait>::function(receiver_if_method, next_arg, ...);


    Point { x: 3, y: 2 }.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
