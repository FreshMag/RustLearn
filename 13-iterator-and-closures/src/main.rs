use std::{env, fs, process, thread};
use std::error::Error;

fn main() {

    // declaring a closure
    let plus_two = |num: i32| -> i32 { // the type is required here because it cannot be inferred
        num + 2
    };

    plus_two(10); // 12

    // COMPARISON
    fn  add_one_v1                (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 =  |x|        { x + 1 };
    let add_one_v4 =  |x|          x + 1  ;

    add_one_v3(5);      // VERY IMPORTANT! Without these two lines, Rust compiler wouldn't be able
    add_one_v4(10);     // to infer the types. The fact that we are using the closures here makes it able to infer

    // -----------------------------

    // Borrowing and Ownership is decided based on what we do within the closure

    // IMMUTABLE borrow

    let list = vec![1, 2, 3];

    let just_print = || println!("Inside closure: {list:?}");

    println!("Before closure: {list:?}");
    just_print();
    println!("After closure: {list:?}");

    // -----------------

    // MUTABLE borrow

    let mut list = list;

    let mut borrow_mutably = || list.push(4);

    // println!("Before closure {list:?}");  // this doesn't compile, because we cannot borrow list as immutable
                                             // while a mutable borrow is still alive.
    borrow_mutably();
    println!("After closure: {list:?}");

    // -----------------

    // TAKING OWNERSHIP

    let list = list;

    let take_ownership = move || println!("Inside closure: {list:?}");  // move keyword

    // println!("Before closure: {list:?}"); // list has been moved! It cannot be used here
    take_ownership();
    // println!("After closure: {list:?}");  // list has been moved! It cannot be used here

    // this is often used for closure executed in new threads, for safety reasons:
    let list = vec![1, 2, 3];

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // if main closed early, it would drop `list`. The compiler requires us to move the value to prevent that

    // --------------------------------------------------------------------

    // Closure and Functions Traits

    // Closure can implement up to 3 of these Traits:

    // `FnOnce`: applies to closures that can be called once. All closures implement at least this
    //           trait because all closures can be called. A closure that moves captured values out
    //           of its body will only implement FnOnce and none of the other Fn traits because it
    //           can only be called once.
    // `FnMut`: applies to closures that don’t move captured values out of their body but might
    //          mutate the captured values. These closures can be called more than once.
    // `Fn`: applies to closures that don’t move captured values out of their body and don’t mutate
    //       captured values, as well as closures that capture nothing from their environment.
    //       These closures can be called more than once without mutating their environment, which
    //       is important in cases such as calling a closure multiple times concurrently.


    // Example with FnOnce

    enum Option1<T> {
        Some(T),
        None
    }

    impl<T> Option1<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T  // because we call it only one, this enables the function `unwrap_or_else`
        {                     // to accept any kind of closure
            match self {
                Option1::Some(x) => x,
                Option1::None => f(),
            }
        }
    }

    // Note: If what we want to do doesn’t require capturing a value from the environment, we can
    // use the name of a function rather than a closure where we need something that implements one
    // of the `Fn` traits. For example, on an `Option<Vec<T>>` value, we could call
    // `unwrap_or_else(Vec::new)` to get a new, empty vector if the value is None. The compiler
    // automatically implements whichever of the `Fn` traits is applicable for a function definition.

    // Example with FnMut

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);  // The reason sort_by_key is defined to take an FnMut
                                               // closure is that it calls the closure multiple
                                               // times: once for each item in the slice.

    println!("{list:#?}");


    // // THIS DOES NOT WORK! The closure we pass here implements only FnOnce, so it won't be accepted
    // let mut list = [
    //         Rectangle { width: 10, height: 1 },
    //         Rectangle { width: 3, height: 5 },
    //         Rectangle { width: 7, height: 12 },
    //     ];
    //
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    //
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    // ..............

    // This instead works since the closure doesn't move any value, it instead maintains only a mutable reference

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

    // ITERATORS --------------------------------------------------------------------

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();  // iterators are lazy!

    for v in v1_iter { // under the hood: took ownership of v1_iter and made it mutable
        println!("{v:?}")
    }

    let v2 = vec![1, 2];
    let mut v2_iter = v2.iter();  // it's `mut`, cause it changes the internal state

    println!("{:?}", v2_iter.next());  // Some(&1)
    println!("{:?}", v2_iter.next());  // Some(&2)
    println!("{:?}", v2_iter.next());  // None

    let sum: i32 = v2.iter().sum();
    println!("{:?}", sum); // consumes the iterator, we cannot use it afterward

    // Iterator methods that produce other iterators

    let _ = v2.iter().map(|x| x + 1);  // this DOES NOTHING UNTIL THE ITERATOR IS CONSUMED

    // for example
    let v3 : Vec<_> = v2.iter().map(|x| x + 1).collect();

    fn less_than(v: Vec<i32>, val: i32) -> Vec<i32> {
        v.into_iter().filter(|x| x < &val).collect()
    }

    println!("{:?}", less_than(v3, 3)); // [2]

    // REWORKING Chapter 12 program

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool
    }

    impl Config {
        fn build(
            mut args: impl Iterator<Item=String>,
        ) -> Result<Config, &'static str> {
            args.next();

            let query = match args.next() {
                None => return Err("Not enough arguments! At least 2 is expected!"),
                Some(v) => v
            };
            let file_path = match args.next() {
                None => return Err("Not enough arguments! At least 2 is expected!"),
                Some(v) => v
            };
            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config { query, file_path, ignore_case })
        }
    };

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    fn run(conf: Config) -> Result<(), Box<dyn Error>> {  // this indicates that the function returns a type that implements the Error trait
        let content = fs::read_to_string(conf.file_path)?; // `dyn` stands for "dynamic", because we don't know the exact type

        pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
            contents.lines()
                .into_iter()
                .filter(|x| x.contains(query))
                .collect()
        }
        // ...

        let res = search(&conf.query, &content);

        for line in res {
            println!("{line}");
        }

        Ok(())
    }

}