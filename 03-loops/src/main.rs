use core::num;

fn main() {
    loop {
        println!("Loops indefinitely!");
        break;
    }

    // Loops can also return values
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Disambuigating loops with labels
    let mut count = 0;
    'outer_loop: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // foreach loops
    let arr = [1, 2, 3];

    for a in arr {
        println!("{a}!");
    }

    // with ranges is even more useful

    for a in (1..3).rev() {
        println!("{a}!");
    }
 

}
