use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();// Returns a smart pointer MutexGuard, wrapped in a
                                        // LockedResult that we unwrap()
        *num = 6;
    }// MutexGuard has a Drop implementation that releases the lock automatically here as num is
     // out of scope

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));// We put our mutex in an atomic Rc<T> (Arc<T>) that is
                                          // thread safe. If we didn't use this the compiler would
                                          // complain because we move the counter value into the
                                          // closure for every thread, so we need something like
                                          // Rc<T> to enable multiple owners
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Similar as with cyclic references when using RefCell<T> and Rc<T>, using mutexes can cause 
// deadlocks. Rust makes you not think about much of the safety concerns, but this is one thing
// that has to be avoided and kept in mind by the programmer!

// Send and Sync traits from std::marker
// Send trait means that the type can be sent (and have its ownership transfered) between threads
// Almost all the primitive types (except for raw pointers, which will be discussed later) have
// Send implemented, but types like Rc<T> and ReffCell<T> do not because they are not thread safe
// Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from
// multiple threads. Any type T is Sync if &T is Send
// Manually implementing Send and Sync involves implementing unsafe Rust code (because they are
// automatically implemented for complex types composed of other types that all implement them)
