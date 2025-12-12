struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    let Point { x: a, y: b} = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis ({x}, {y})"),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    enum Message { Hello { id: i32 } };

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { 
            id: id_variable @ 3..=7,//using @ here we check if id is in range and capture iits
            //value into a variable id_variable
        } => println!("Found an id in range 3-7: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!("Found an id in range 10-12"),
        Message::Hello { id } => println!("Some other id {id}"),
    }
}

