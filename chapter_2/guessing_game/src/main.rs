use std::io;
use rand::Rng;
use std::cmp::Ordering;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>());
//}

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    println!("--DEBUG-- The number is {}", number);//DEBUG
     
    println!("Guess a number");
  

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Guess is {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
               println!("Correct!");
               break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
