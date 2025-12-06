fn main() {
    another_function(16, 's');
    another_function(5, 'o');
    another_function(plus_one(23), 'p');
    another_function(plus_one(0), 'p');
}

fn another_function(x: i32, c: char) {
    println!("Value of x is {x}, char c is {c}");
}

fn plus_one(x: i32) -> i32 {
    if x == 0 {
        return 0 //you can return early with return
    }
    x + 1 //last expression without a ; represents the return value
}
