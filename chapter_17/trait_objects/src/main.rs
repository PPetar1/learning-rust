fn main() {
    let screen = Screen {
       components: vec![      
             Box::new(SelectBox {
                 width: 75,
                 height: 10,
                 options: vec![
                     String::from("A"),
                     String::from("B"),
                 ],
             }),
             Box::new(Button {
                 width: 55,
                 height: 2,
                 label: String::from("L"),
             }),
        ],
    };

    screen.run();
}

pub trait Draw {
    fn draw(&self);
}
// When using dyn the code for the method called is determined at runtime!
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,// Here we create a vector containing Box pointers to
                                       // types that implement the Draw trait. This is different
                                       // from generic types because vector can store multiple
                                       // types at the same time and we dont have to determine 
                                       // at the time we are writing this all the possible types
                                       // that can be stored here. This is different from regular
                                       // polymorphism in a sence that we can only use a method of
                                       // another type, not its data!
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    height: u32,
    width: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //some code
    }
}

struct SelectBox {
    height: u32,
    width: u32,
    options: Vec<String>,
}


impl Draw for SelectBox {
    fn draw(&self) {
        //some code
    }
}

