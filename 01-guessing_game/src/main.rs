use rand::Rng;
use std::io; // Notice we are not using the `Rng` name. This is because Rng must be in scope for use to use methods
             // for random number generation
             // use `cargo doc --open` to see the documentation of all crates

use std::cmp::Ordering;

fn main() {
    println!("Indovina il numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // lower and upper inclusive

    loop {
        println!("Per favore inserisci il tuo tentativo:");

        let mut guess = String::new(); // Variables are IMMUTABLE by default. We need `mut` explicitly

        io::stdin() // returns a std::io::Stdin instance
            // a `&` is a REFERENCE to the variable, allowing to not copy the content of guess
            // References are IMMUTABLE like variable, so here we need `mut` as well
            .read_line(&mut guess) // NOTE: it APPENDS to `guess`, doesn't overwrite
            // `read_line` returns a Result, which is an enumeration. Each possible state of a enum is called `variant`
            // variant of Result are `Ok` and `Err`
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // Here type is needed since `parse` is generic
        // NOTICE: we are SHADOWING the previous `guess` variable.
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        //println!("String interpolation in println! is even more powerful!");
        //let x = 5;
        //let y = 10;
        //println!("x = {x} and y + 2 = {}", y + 2);

        // Match expression
        match guess.cmp(&secret_number) {
            // Ordering is another enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }
}
