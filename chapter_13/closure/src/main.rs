// Closures
use std::thread;

fn main() {
    // Defining closures has similar syntax as defining functions
    //fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    //let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x|             { x + 1 };
    //let add_one_v4 = |x|               x + 1  ;
    
    
    let example_closure = |x| x;
    
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5);// This code won't compile, compiler already deduced the return type
                                 // of example closure to be a string from the line above
    let mut e = ExampleStruct { x: 3 };
    
    let testing_closure_referencing = || e.x;// Closures can capture environment variables like this,
                                             // when they do they are borrowed
    
    //e.x_add_one();// So here we cannot do this because we are taking a mut reference while the
                    // closure still holds an immutable one
    
    testing_closure_referencing();
    
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || {// Here we spawn a new thread and give away the ownership of the Vec
                           // list by puting move before the closure definition; If we didn't
                           // specify move we would get a compiler error - this is because main
                           // could finish before the thread and if it would still had the
                           // ownership of list it would invalidate the reference
        println!("From thread: {:?}", list)
    }).join().unwrap();// We wait for the execution of the thread and unwrap() the contents

    let mut list = [
        Rectangle { width: 10, height: 1 },      
        Rectangle { width: 3, height: 5 },      
        Rectangle { width: 7, height: 12 },      
    ];
    
    list.sort_by_key(|r| r.width);// The sort_by_key function is defined to take FnMut closure
                                  // because it calls it multiple times, once for each item in the
                                  // slice. The function passed doesn't have to change the
                                  // arguments, but it could if we needed it to

    //let value = String::from("string");
    //let sort_operations = vec![];

    //list.sort_by_key(|r| { 
    //    sort_operations.push(value);
    //    r.width
    //});// This won't work because the closure is FnOnce only because the first time it is ran it
         // will transfer ownership of value parameter to the sort_operations list and after that
         // it will be invalid to call it again, but sort_by_key needs to call the closure multiple
         // times (once for each item in the slice)

    println!("{:#?}", list);
}

struct ExampleStruct {
    x: i32,
}

impl ExampleStruct {
    fn x(&self) -> i32 {
        self.x
    }

    fn x_add_one(&mut self) -> () {
        self.x += 1;
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The way a closure captures and handles values from the
// environment affects which traits the closure implements, and traits
// are how functions and structs can specify what kinds of closures
// they can use. Closures will automatically implement one, two, or all
// three of these Fn traits, in an additive fashion, depending on how the
// closure’s body handles the values:
// FnOnce applies to closures that can be called once. All closures
// implement at least this trait because all closures can be called. A
// closure that moves captured values out of its body will only
// implement FnOnce and none of the other Fn traits because it can only
// be called once.
// FnMut applies to closures that don’t move captured values out of their
// body, but that might mutate the captured values. These closures can
// be called more than once.
// Fn applies to closures that don’t move captured values out of their
// body and that don’t mutate captured values, as well as closures that
// capture nothing from their environment. These closures can be
// called more than once without mutating their environment, which is
// important in cases such as calling a closure multiple times
// concurrently.


