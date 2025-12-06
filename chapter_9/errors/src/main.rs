use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("Error message");// This macro causes the program to panic with the given error mess.    
    let v = vec![1, 2, 3];

    // v[99];// This causes the program to panic; the panic call is made in the std lib files
    
    // If we set env var while runing cargo with RUST_BACKTRACE=1 cargo run we can look at
    // backtrace

    // Recoverable errors with Result
    // enum Result<T,E> {
    //      Ok(T),
    //      Err(E),
    // }


    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem creating the file: {:?}",
                        e
                    ),
                }
            }
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        }
    };
    
    // Same thing using closures, more about them will be covered later
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //let greeting_file = File::open("hello1.txt").unwrap();// unwrap() method returns the value if
                                                          // the Result is Ok(value) or panic! if
                                                          // the Result is an Err

    //let greeting_file = File::open("hello1.txt").expect("Error message");// expect() is the same as
                                                                         // unwrap() it just takes
                                                                         // a custom error message
                                                                         // as an argument to be
                                                                         // passed to panic!()

   


}
//Propagating errors

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("file.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//Propagating errors using the ? operator

fn read_username_from_file_alt() -> Result<String, io::Error> {
    let mut username_file = File::open("file.txt")?;// ? returns the value inside the Ok or returns
                                                    // from the func if Err is returned
                                                    // Only difference from the method above is
                                                    // that ? makes error values go through from
                                                    // function which converts the error into the
                                                    // return error type for our function


    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    // File::open("hello.txt")?.read_to_string(&mut username)?;// This can be chained together alternatively
    Ok(username)
}

use std::fs;
fn read_username_from_file_alt_2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")// std lib provides a function to read file contents into string
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()// When ? is used on an Option it evaluates to value from
                                      // Some(value) or returns None early from the function
}

// To use ? in the main function you need to make main return fn main() -> Result<(), Box<dyn Error>> and exit it with Ok(()). Box<dyn Error> is some sort of a trait object which will be covered later
