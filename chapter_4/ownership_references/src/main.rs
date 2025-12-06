fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //calculate_lenghth takes a reference to s1; ownership is
                                     //never transfered; we say that the function is borrowing s1
    println!("The length of '{s1}' is {len}."); // We can still use s1 here because we passed the
                                                // reference only

    //change (&s1); 
}
fn calculate_length(s: &String) -> usize {// s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
 // it refers to, the String is not dropped.

//fn change(some_string: &String) { // Doesn't work, references are immutable by default
//    some_string.push_str(", world");
//}
