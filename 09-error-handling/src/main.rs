use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // panic!("PANICKED AAAA") // this is a UNRECOVERABLE error. The program panics and then exits
                            // without alternative

    // RECOVERABLE ERRORS: errors that don't mean there is a bug, errors you can interpret
    let greeting_file_result = File::open("hello.txt"); // this is a result

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}")
    };

    let greeting_file_result = File::open("hello.txt"); // this is a result
    // we can handle this better
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {e:?}")
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        }
    };

    // the Result class has a lot of useful methods
    let greeting_file = File::open("hello.txt")
        .expect("File not opened."); // this is the message of panic!

    // example of writing functions that return Result
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("username");

        let mut username_file = match username_file_result {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // but we can do even more using the '?' operator!
    fn read_username() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("username")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // the '?' operator does what we have done with the 'match' expressions early, except that
    // it tries to convert the Result into another Result based on the return type of the current
    // function. This is possible using the `From` trait, that enables conversion between types.
    // if the Result is an Err, it returns that Result.

    // The '?' operator works with Option too!
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

}
