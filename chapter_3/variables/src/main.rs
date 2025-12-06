fn main() {
    let mut x = 5;
    
    let x = x + 1;
    {
        let x = x * 2;
        println!("Value of x in the inner scope is: {x}");
    }
    println!("Value of x is: {x}");

    //types in rust - i8,u8,i16,u16...i128,u128,isize,usize;f32,f64;bool;char(Unicode)

    let tup: (i32, f64, bool) = (12, 23.4, true);
    let (x,y,z) = tup;//deconstruction

    println!("Tuple: ({x}, {y}, {z})");//can also use tup.0, tup.1, tup.2

    let array: [i32;5] = [1,2,3,4,5];// array type - on stack, fixed length, same type
    let a = [5; 5];//makes an array [5,5,5,5,5]

    //a[0] = 3; //causes an error, elements are imutable by default

    println!("{}", a[0]);

}
