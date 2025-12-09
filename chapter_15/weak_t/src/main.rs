// Weak<T> - used to create weak references, those wont count when Rc decides whether to drop
// or not; Because of this, they can point to something that has been deallocated, we need to
// use upgrade method and handle the returning Option<Rc<T>> that will tell us if the value
// still exists or not

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,// We want the child to hold the reference to the parent as well
                                // but using Rc<T> would create a reference cycle, so we use
                                // Weak<Node>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

            
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

    }

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );
}
