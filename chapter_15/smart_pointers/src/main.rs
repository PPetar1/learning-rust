// Reference counting - Rc<T>

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}",
        Rc::strong_count(&a)
    );
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}",
        Rc::strong_count(&a)
    );
    {
        let c = Cons(4, Rc::clone(&a));// We wouldn't have been able to do this without Rc, we would
                                   // have to pass ownership of a two times. Rc here allows for
                                   // shared ownership of a, and a will go out 
                                   // of scope when all the references go out of scope. Rc::clone
                                   // is preferable to use instead of a.clone because it is clear
                                   // then that we are making a shallow copy of the data, actually
                                   // we just increase the reference count
        
         println!("count after creating c = {}",
             Rc::strong_count(&a)
         );
    }

    println!("count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );


    // RefCell<T> - allowing multiple mutators to the same data - important this uses unsafe code!

    // Interior mutability is a design pattern that allows you to mutate data even when there are
    // immutable references to that data
    // We can use types that use the interior mutability pattern only when
    // we can ensure that the borrowing rules will be followed at runtime,
    // even though the compiler can’t guarantee that. The unsafe code
    // involved is then wrapped in a safe API, and the outer type is still
    // immutable.
    // RefCell<T> enforces borrowing rules at runtime, if we break them the program will panic!
    // Rc<T> and RefCell<T> are both only for use in single-threaded scenarios, for multi-threading
    // there are other ways that will be disscused later
}

pub trait Messenger {
    fn send(&self, msg: &str);
}   

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(
        messenger: &'a T,
        max: usize
    ) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    pub fn set_value(&mut self, value: usize){
        self.value = value;
        let percentage_of_max =
            self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger
            .send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
            .send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
            .send("Warning: You're at 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //self.sent_messages.push(String::from(message));// This wont work because we are taking
                                                           // a immutable ref to self and then
                                                           // changing interior data, we cannot
                                                           // change the function signature because
                                                           // we will not be able to then implement
                                                           // the messenger trait that we are
                                                           // testing (real function wont modify
                                                           // internal data here but only send it)
                                                           // so we will change the implementation
                                                           // to use RefCell

            self.sent_messages.borrow_mut()// unsafe borrow
                              .push(String::from(message));

           // This code panics because we have 2 mutable borrows
           // let mut one_borrow = self.sent_messages.borrow_mut();
           // let mut two_borrow = self.sent_messages.borrow_mut();
           // 
           // one_borrow.push(String::from(message));
           // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100
        );
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// We can combine Rc and RefCell to have a value that can be owned and modified by multiple owners
