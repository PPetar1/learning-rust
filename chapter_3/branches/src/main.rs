fn main() {
    let number = 3;
    if number < 3 { //if number { wont work, condition has to be bool
        println!("number is less than 3");
    }
    else if number == 3 {
        println!("number is 3");
    }
    else {
        println!("number is larger than 3");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // you can do this because if is an expression
    // let number = if condition { 5 } else { "string" } // wont work due to type missmatch
    println!("The value of number is: {number}");
}
