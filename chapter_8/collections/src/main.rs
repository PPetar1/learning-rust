
fn main() {
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




}
