// Paterns that will match any possible value are refutable (let x = 5)
// Example of a refutable pattern is if let Some(x) = a_value (this doesn't match if a_value is
// None)
// Function parameters, let statements and for loops only accept irrefutable patterns
// if let and while let accept both refutable and irrefutable patterns, but the compiler will warn
// for irrefutable because they are by design conditional
// match must use refutable patterns except for the last arm that should match any remaining values
// with an irrefutable pattern

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something_else"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
   
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("something_else"),
    }

    let x = 5;

    match x {
        1 ..= 5 => println!("one trough five"),
        _ => println!("something_else"),
    }
}

