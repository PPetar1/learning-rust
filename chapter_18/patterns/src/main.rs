// Patterns

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as back col");
        }
        else {
            println!("Using orange as back col");
        }
    } else {
        println!("Using blue as back col");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let (x, y, z) = (1, 2, 3);

    let point = (3, 4);

    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
