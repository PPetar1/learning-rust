// macro_rules!

#[macro_export]// This indicate that whenever our crate is brought into the scope the macro should
               // as well
macro_rules! vec {// This starts the definition of vec! macro
    ( $( $x: expr ),* ) => {// This means there is unknown number of somethings separated by a
                            // comma and somethings are made of a named macro variable x that
                            // contains any rust expresion
                            // https://doc.rust-lang.org/stable/reference/macros-by-example.html
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procedural macros
//https://doc.rust-lang.org/stable/book/ch20-05-macros.html
fn main() {

}
