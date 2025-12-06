struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;// Unit-like struct (similar to ()), it doesn't have any data but can have
                   // traits which will be covered later
                
fn main() {

    let subject = AlwaysEqual;// This is how you create an instance of a unit-like struct

    let mut user1 = User { // The whole instance must be mutable, you cannot mut just one field
        active: true,
        username: String::from("username1"),
        email:String::from("email1"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail1");

    let user2 = build_user("email2".to_string(),"user2".to_string());


    let user3 = User {
        email: String::from("email3"),
        ..user1 // This means that all the fields except email will have the same value as those
                // from user1
                // This moves all the types that don't have the copy trait, so referencing
                // user1.username would cause an error from now on
    };

    println!("{}, {}", user1.email, user2.email);
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // has the same meaning as username: username, this shorthand can be used due to
                 // the same name of the field and the parameter
        email, //has the same meaning as email: email,
        sign_in_count: 1,
    }
}
