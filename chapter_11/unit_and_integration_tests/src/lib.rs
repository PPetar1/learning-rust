pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]// This annotation tells the compiler to compile and run the code in this module only when the cargo test is ran, not when you do cargo build 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
