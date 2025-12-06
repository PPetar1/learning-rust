#[derive(Debug)]// attribute that tells the compiler to auto implement the Debug trait
                // you use derive to add other traits as well i think, but this will be covered in
                // the future; derive is just one of multiple attributes
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        rectangle.width < self.width && rectangle.height < self.height
    }

    fn sqare(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    // Solution 1
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    //Solution using tuples

    //let rect1 = (30, 50);

    //println!(
    //    "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    //);
    
    //Solution using structs
    
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
         rect1.area()
    );

    println!("rect1 is {:#?}", rect1);// {:?}, {:#?} prints debug info and we can use it if we add
                                      // #[derive(Debug)] to the struct; {} wont work because we
                                      // dont implement std::fmt::Display
    
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),// dbg! is a macro that takes ownership of the expresion, prints
                                // its value to stderr and then returns ownership
        height: 50,
    };

    dbg!(&rect1);// Here we pass forward reference to rect1 because we dont want to lose ownership
    let rect1 = dbg!(rect1);// I think we can also do this
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::sqare(4);
    dbg!(rect4);
}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}

// This function is moved to be the method of the Rectangle struct
//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.height * rectangle.width
//}
