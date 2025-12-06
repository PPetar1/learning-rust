enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),// The value constructor can also use a struct (or another enum) so std lib for 
               // example uses Ipv6Addr and Ipv4Addr and then has enum with 
               // V4(Ipv4Addr), V6(Ipv6Addr)
    example {x: u32, y: u32},// You can also define structs here
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    //Option<T> is an enum with None, Some(T)
    let some_number = Some(6);// type is Option<i32>
    let some_char = Some('e');// type is Option<char>

    let absent_number: Option<i32> = None;// here we have to specify the type since it cannot be
                                          // determined from None alone
}



