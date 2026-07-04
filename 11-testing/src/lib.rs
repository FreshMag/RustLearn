pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn panic_park() {
    panic!("Welcome to panic park!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Welcome to panic")] // this is a substring of the panic message
    fn test_panic() {
        panic_park()
    }

    // we can also take advantage of the Result class to build tests
    #[test]
    fn test_result() -> Result<(), String> {
        let v = add(2, 3);

        if v == 5 {
            Ok(())
        } else {
            Err(String::from("The result was not 5"))
        }
    }

    #[test]
    #[ignore]
    fn test_ignored() {}
}
