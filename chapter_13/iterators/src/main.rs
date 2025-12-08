// Iterators
fn main() {
    let mut v1 = vec![1, 2, 3];
    
    let mut v1_iter = v1.iter();// Here we create an iterator on v1

    println!("{}", v1_iter.next().unwrap());// We can call next on an iterator but need to make it
                                            // mutable first because we change internal iterator
                                            // state; for loop doesn't need mut because it takes
                                            // ownership and makes it mutable in some way
    for val in v1_iter {
        println!("Got: {val}");
    }

    // Here we make iterators that return owned values and mut references. The default iter returns
    // immutable references
    let v1_iter_returning_owned_values = v1.clone().into_iter();

    let v1_iter_returning_mut_ref = v1.iter_mut();

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();// Here we use the fn sum which is implemented by the std lib by
                              // default. It takes ownership of the iter and consumes it (because
                              // it uses next()), so using v1_iter after this will fail, functions
                              // like that are called consuming adapters

    let v1: Vec<i32> = vec![1, 2, 3];

    let map_iter = v1.iter().map(|x| x + 1);// These methods are called iterator adapters, they do not consume an
                             // iterator but instead pass a new one back to us; If we do not
                             // consume the iterator, the closure will never get called because
                             // iterators are lazy
    let maped: Vec<_> = map_iter.collect();
                             
    println!("{:#?}", v1.iter().filter(|e| **e < 3).collect::<Vec<_>>());// A little bit of a
                                                                         // cursed syntax that will
                                                                         // probably be covered
                                                                         // later but the main
                                                                         // point is you can use
                                                                         // filter; Not sure why do
                                                                         // we have e of type &&i32
                                                                         // though

}

// In order to implement Iterator std lib trait we only need to implement the next funtion (and the
// Item type?)
pub trait Iterator_ {// How std lib implements iterators
    type Item;// This is called associated type and will be covered later; here it means that
              // anyone implementing this trait needs also to implement Item type (or something
              // like that)
    fn next(&mut self) -> Option<Self::Item>;// Here we return that Item type

    // methods with default implementations elided
}

// Iterators and closures are implemented so that they cause zero performance cost, it should
// compile down to the same code as when not using them
