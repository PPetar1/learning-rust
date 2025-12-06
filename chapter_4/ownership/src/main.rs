fn main() {
    let x = 5;
    let y = x;

    println!("x is: {x}, y is {y}");

    let s1 = String::from("hello");
    let s2 = s1; // A move is occuring here; s1 is no longer valid and s2 becomes the owner of the string; String type doesnt implement Copy trait (Copy trait is an annotation that can be placed on types that are stored on the stack, so String here cannot implement it (if I'm understanding this correctlly))

    //println!("{s1}"); // This returns an error, s1 is not valid after s2 is declared
    println!("{s2}");

    let mut s3 = String::from("hello");
    let mut s4 = s3.clone(); // This preforms a deep copy of the string, now we have 2 strings in the heap that both contain "hello"

    println!("s3 = {s3}, s4 = {s4}"); // Both s3 and s4 are valid here

    s4.push_str(", world"); // We change s4
    println!("s3 = {s3}, s4 = {s4}"); // And s3 is not affected


}
