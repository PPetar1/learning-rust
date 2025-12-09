// Box<> pointers
// Data inside gets stored on the heap instead of the stack
// Box pointers are smart pointers but they don't have any other
// special capabilities except indirection and heap allocation
// Because of that they do not have any performance overhead
// except for using heap rather than a stack
// They do however implement the Deref and Drop traits causing
// us to be able to use them as references and Drop does deallocation

use crate::List::{Cons, Nil};
use crate::List_::{Cons_, Nil_};

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    assert_eq!(5, *b);
   
    let m = MyBox::new(5);

    assert_eq!(5, *m);
    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );

    let list = Cons_(
        1,
        &Cons_(
            2,
            &Cons_(
                3,
                &Nil_
            )
        )
    );

    fn hello(name: &str) {
        println!("Hello, {name}");
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);// Deref coercion happens here - reference to MyBox get converted to a reference to
              // String that gets converted to a reference to str. This happens automatically with
              // tyoes that implement Deref trait
    println!("{}", m.deref());
    println!("{}", (&m).deref());// Not quite sure why does this work in both ways, maybe
                                 // self.something() can auto convert to &self.something() but it
                                 // will be covered later

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };

    //c.drop();// This code wont compile, we cannot call drop() manually because Rust would preform
               // double clear when the variable goes out of scope
    drop(c);// This calls std::mem::drop func that is in the prelude and it somehow prevents the
            // compiler to clear later
    println!("CustomSmartPointer droped before the end of main");
    //c.data;// This doesn't work because you transfer ownership
}

enum List {// Recursive type
    Cons(i32, Box<List>),// If we didn't use Box here, the compiler woudn't have a way to determine
                         // how big is our type due to infinite recursion. When we use Box<List> he
                         // knows that the space needed is for one i32 and one Box pointer
    Nil
}

enum List_<'a> {// This implementation would also work
    Cons_(i32, &'a List_<'a>),
    Nil_,
}

// Deref trait

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// There is also DerefMut for mutable references
// Rust does deref coercion when it finds types and trait
// implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U


// Drop trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Droping CustomSmartPointer with data `{}`!",
            self.data
        );
    }
}
