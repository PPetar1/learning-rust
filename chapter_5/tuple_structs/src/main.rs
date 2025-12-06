struct Color(i32, i32, i32);// Tuple structs dont have names for their fields, and are sort of a
                            // named tuple that is its own type. This means that for example if a
                            // function takes Color as its argument it cannot take Point as well
                            // even though they have the same types for their fields
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    println!("{}, {x} {y} {z}", origin.0);// You can use special tuple syntax with tuple structs
}
