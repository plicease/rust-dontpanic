#[no_mangle]
pub extern "C" fn answer() -> i32 {
    42
}

#[cfg(test)]
mod tests {

    use crate::answer;

    #[test]
    fn it_works() {
        assert_eq!(answer(), 42);
    }
}
