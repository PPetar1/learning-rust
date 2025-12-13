// Function pointers (fn)
// Most of the time though we want to use a generic type that implements one of the Fn traits so
// that the closures can work as well

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

// Returning closures

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let answer = do_twice(add_one, 5);

    assert_eq!(answer, 7);
}
