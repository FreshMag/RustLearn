use std::os::macos::raw::stat;

enum IpAddrKind {
    V4,
    V6
}

// works, but is it the best?
struct Ip1 {
    kind: IpAddrKind,
    address: String
}

// data right inside the enum!
enum IpAddr {
    V4(String),
    V6(String)
}

// We can be even more flexible!
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// as for structs, we can define methods for enums
impl Message {
    fn call(&self) {
        // method body
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin2 {
    fn value_cents(&self) -> u8 {
        match self {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let new = IpAddr::V4(String::from("127.0.0.1"));

    let message = Message::Write(String::from("Hello"));
    message.call();

    // and now the most useful enum of all. `Option`
    let opt = Some(5);
    let absent: Option<i32> = None;

    //
    let my_coin = Coin::Quarter;
    println!("Value is {}", my_coin.value_in_cents());

    let my_coin2 = Coin2::Quarter(UsState::Alaska);
    println!("Value is {}", my_coin2.value_cents());

    // match must be exhaustive! We can cover missing values with a general pattern:
    match my_coin2 {
        Coin2::Quarter(state) => {
            println!("State is: {:?}", state);
            ()
        },
        _ => (),
    }

    // What if we want to execute this same code but in a more concise way?
    // We have the `if let` statement!
    let i = Some(5);

    if let Some(x) = i {
        println!("i is actually a Some with {} in it!", x)
    }
    let my_coin3 = Coin2::Quarter(UsState::Alaska);

    if let Coin2::Quarter(state) = my_coin3 {
        println!("The state is {:?}", state)
    } else {
        println!("The coin is not a quarter!");
    }

    // this statement is also an expression! It allows destructuring:
    let job = Some("programmer");
    let job_name = if let Some(name) = job {
        name
    } else {
        "unemployed"
    };

    // or running backup code if it doesn't match
    fn print_job(job: Option<&str>) -> Option<String> {
        let Some(job_name) = job else {
            return None
        };
        Some(format!("'{job_name:?}' is your job!"))
    }
    println!("Job is {:?}", print_job(job))
}
