fn main() {
    let number_list = vec![34, 22, 100, 50];

    let result = largest(&number_list);// Here the compiler can deduce we are talking about
                                       // largest<i32> because number_list is [i32]

    println!("The largest number is {result}");
}

// Generic parameters in functions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {// Here we use the generic type T in the
                                                       // functions signature; We tell the compiler
                                                       // that the type must implement the
                                                       // std::cmp::PartialOrd trait because we are
                                                       // comparing values of that type in the code
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic parameters in structs

struct Point<T> {
    x: T,
    y: T,
}

// Generic parameters in enums

enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}


// Generic parameters in methods

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

   // fn x(&self) -> i32 { // This doesn't work, there probably is a way to overwrite fun def
   //     32
   // }
}

struct Point_<X, Y> {
    x: X,
    y: Y,
}

impl<X1, Y1> Point_<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point_<X2, Y2>) -> Point_<X1, Y2> {// Here we use the structs
                                                                     // generic types (X1, Y1) and
                                                                     // the method itself has
                                                                     // generic types (X2, Y2)
        Point_ {x: self.x, y: other.y}
    }
}

// Generics are implemented like so - the compiler checks the code and finds all the uses of
// generic functions and creates all the separate types of those based on the types that were used
// in the code. Rust probably has some dynamic types as well and for those i guess you just cant
// use generics, we will see. They call this process monomorphization and it obviously doesn't
// incure any runtime cost


// Traits


