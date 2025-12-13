// Type synonyms and type aliases

type Kilometers = i32;

// There is a "never type" ! that signifies a return type that should never be returned, for
// example its the return value of the panic! macro

// Dynamically sized types are those whose size cannot be known at compile time (like str)
// All the types whose size is known at compile time implement Sized trait, DSTs do not
// DSTs must always be in a pointer of some kind for us to be able to use them
// Generics are implicitly types that implement Sized, if we want to bypass this restriction we
// must use fn generic<T: ?Sized> (t: &T); ?Sized syntax is only available for the Sized trait
fn main() {
    let x: i32 = 5;
    let y: Kilometers  = 5;// type synonym only lets you call the type by another name, in all
                           // other regards it is the exact same type
                           // Main use is to reduce repetition of lengthy types

    assert_eq!(x + y, 10);
}
