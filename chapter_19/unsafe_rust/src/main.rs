// You switch to unsafe rust by using an unsafe {} block
// There are 5 actions you can take in unsafe Rust that you can't in normal
// Dereference a raw pointer
// Call an unsafe fn or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions
use std::slice;

fn main() {
//Different from references and smart pointers, raw pointers:
//Are allowed to ignore the borrowing rules by having both immutable
//and mutable pointers or multiple mutable pointers to the same location
//Aren’t guaranteed to point to valid memory
//Are allowed to be null
//Don’t implement any automatic cleanup    

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;// We can create raw pointers in safe code, just not derefence
                                  // them

    let address = 0x01234usize;// This is only for demonstration purposes
    let r = address as *const i32;// We cannot be sure what this points to

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        //println!("r is: {}", *r);// Our first seg fault in Rust!!
    }

    // Unsafe fn
    
    unsafe fn dangerous() {}

    unsafe { 
        dangerous();
    }

    // Safe abstraction over unsafe code

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();

        let ptr = values.as_mut_ptr();// Returns a *mut i32

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),// We create a slice of len mid
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),// We create a
                // slice of len len - mid starting at ptr + mid
            )
        }
    }

    // Using extern functions to call external code
    
    unsafe extern "C" {// We call external function (from C std lib). This is generally unsafe because
                       // other languages don't follow Rust guarantees
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or modifying a mut static variable
    static HELLO_WORLD: &str = "Hello world!";// Rust has global variables and they are declared
                                              // and called static. They only can store references
                                              // with the 'static lifetime. Accessing and modifying
                                              // mutable static variables is unsafe

    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", *(&raw const COUNTER));// println! takes a reference and we need to
                                                       // make one explicitly using raw borrow
                                                       // operators
    }

    // Implementing an unsafe trait
    // Unsafe traits are those where at least one of its methods has some invariant that the
    // compiler can't verify

    unsafe trait Foo {

    }

    unsafe impl Foo for i32 {

    }

    // Unions are mostly used to interface with unions in C code
}

