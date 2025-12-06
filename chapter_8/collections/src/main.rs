fn main() {
    //-----------------------------------------------------------------------------------------
    // Vectors (Vec<T>)
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];// vec! macro creates a vector from values and figures out the type

    let mut v3 = Vec::new();// Here the type is figured out from later statements, which is 
                            // kind of weird

    v3.push(3);
    v3.push(4);
    
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];// If we access an element that is out of bounds this way (v[100])
                            // the program will panic
    let fourth: i32 = v[3];// This should also work because int has Copy trait but is probably a
                           // bad idiom because in Vec<T> T can be a type that doesnt implement a
                           // Copy trait; also working with references mean you cannot get into a
                           // situation where the vector is changed but your copy still follows an
                           // older version
 
    //v.push(3);// This would cause a program to panic - we do a mutable and an immutable borrow 
              // and their scopes overlap, this is not allowed (even for vectors because we might
              // need to alocate extra space after a push and that could move other elements as
              // well so our pointer/reference would become invalid - but it also panics for
              // v.pop() so still not entirely sure how it works)
    println!("{third}{fourth}");

    let third: Option<&i32> = v.get(2);// In the case when we try to get an element that is out of
                                       // bounds get method returns None which we have to handle
                                       // when unwraping Option type
    match third {
        Some(third) => println!("{third}"),
        None => println!("There is no third element."),
    }

    // Iteration with for loop
    
    for num in &v {
        println!("{num}");
    }

    for num in &mut v {
        //num += 50; // This doesn't work
        *num += 50;
        println!("{} = {}", num, *num);// But this does, so automatic dereferencing sometimes
                                      // happens sometimes not, i have to pay atention to see when
                                      // and why
    }

    // Storing multiple types in a vector by using enums
    
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("text")),
        SpreadsheetCell::Float(10.12),
    ];
    
    // Could be useful in the future to review the API documentation for all the collections

    //-----------------------------------------------------------------------------------------
    //Strings (String)

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // Or
    let s = "initial contents".to_string();

    // Or
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    let mut s2 = "bar";
    s.push_str(s2);

    println!("s2 is {s2}");
    s2 = "par";

    println!("s is {s}");

    println!("s2 is {s2}");// Not sure how is this working, you probably push a reference to a
                           // literal to s instead of a reference to s2

    let s1 = "hello".to_string();
    let mut s2 = ", world".to_string();

    let s1 = s1 + &s2;

    println!("{s1}{s2}");
    s2.push_str("testing");
    
    println!("{s1}{s2}");// From what i can tell both push_str() and add() (used by +) COPY the
                         // contents of the reference passed by the second argument into the first
                         // one (or self for push_str()). After let s1 = s1 + &s2 s1 has its own
                         // copy with contents s1_olds2 and s2 has its own memory with just s2


    let s = format!("{s1} --- {s2}");
    println!("{s}\n{s1}\n{s2}");// Format macro opposed to methods used above, doesn't move the
                                // first string, so here both s1 and s2 are still usable

    // You cant use indexes with strings because they are encoded in Unicode and can be of varying
    // length, so Rust std lib didn't implement indexing to avoid confusion
    // You can use string slicing so let hello = "Здравствуйте"; let s = &hello[0..4]; is valid
    // code in Rust, but s = &hello[0..1] would fail because cyrilic uses 2 bytes per character 
    // and you cannot slice just part of a char. This does still expose internal representation to
    // you so they probably just didn't figure out how to make slicing work and prevented just the
    // indexing
    
    // The intended way to work with chars of a string is to use char type
    
    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    for b in "Здравствуйте".bytes() { // If we want to extract bytes individually
        println!("{b}");
    }

    println!("{}{}", s.contains('t'), s.replace("t", "T"));// Some useful std lib func for working
                                                           // with String


    //-----------------------------------------------------------------------------------------
    // Hash maps (HashMap<K, V>)

    use::std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);// copied() makes an Option<i32> from
                                                             // Option<&i32> and unwrap_or(0)
                                                             // unwraps the Option getting the
                                                             // value or setting it to 0 if it is
                                                             // None

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    let key = String::from("key");
    let value = String::from("value");

    let mut map = HashMap::new();

    map.insert(key, value);

    // key and value are not valid here anymore, the map takes the ownership, using them would
    // cause panic


    let key = String::from("key");
    let value = String::from("value");
    let another_value = String::from("another_value");

    map.insert(key, value);// This overwrites the previous value stored with the key

    // Using Entry enum

    scores.entry(team_name).or_insert(50);// This inserts 50 only if there isnt an entry for the key;
                                 // otherwise it returns a mutable reference to the value for the
                                 // corresponding Entry key
    

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
