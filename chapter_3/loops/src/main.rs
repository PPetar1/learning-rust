fn main() {
    // rust has 3 kind of loops - for, while and loop
    
    let mut x = 1;
    let result = loop {
        println!("{x}");
        x = x + 1;
        if x > 20 { break x * 2}
    };

    println!("Result is {result}");


    //loop labels
    
    let mut count = 0;
    'counting_up: loop { //outer loop is labeled 'counting_up (labels must start with ')
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //this stops the inner and the loop labeled 'counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    //while loops
    
    let mut number = 3;

    'while_loop: while number != 0 { //labels work with wile and for as well
        println!("{number}");
        number -= 1;
    }

    //for loops
    
    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
