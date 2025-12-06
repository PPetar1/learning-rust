#[derive(Debug)]
enum UsState {//Boze zasto
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => 12,
        5 => 32,
        other => other + 1,// Catch all pattern, for unnamed ones you can use _ and if you dont
                           // want it to do anything you can return a () so _ => ()
    };

    let option = Some(3u8);
    if let Some(value) = option {//This is the same as using a match that does something only if
                                 //there is a value and ignores if it is None (or in general case
                                 //anything other than Some()); You can also specify else and that
                                 //would execute if the expresion (option) matches to something
                                 //other than Some()
        println!("do some code");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
                println!("Lucky penny(sta god to znacilo)");
                1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),// match has to handle all cases, if we excluded a case this wouldn't
                             // compile
    }
}
