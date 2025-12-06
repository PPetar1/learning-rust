fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s;
    //let r2 = &mut s; 
    //println!("{r1}, {r2}"); // This code fails because you cannot have multiple mutable references at the same time

    let r3 = &s; // no issue (r1 is not in scope here since its not used bellow)
    let r4 = &s; // no issue, multiple immutable references are allowed

    println!("{r3}, {r4}");

    let r5 = &mut s;

    println!("{r5}");
    //println!("{r3}, {r4}"); // This wont compile; it extends the scope of r3 and r4 so their scope
                            // overlaps with that of r5 which is not allowed - mut and imut ref
                            // cannot be used at the same time
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
