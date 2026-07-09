use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fmt::format;
use std::io;
use rand::RngExt;

#[allow(dead_code, unused_imports, unused)]

fn main() {
    {
        let v: Vec<i32> = Vec::new(); // static method
    }

    {
        let v = vec![1, 2, 3]; // macro to build a new Vec easily
    }

    {
        // Accessing the vector

        let mut v = vec![];

        v.push(3); // test-workspace new elements to an existing vector
        v.push(4);

        println!("{:?}", v);

        // Accessing elements inside the vector: TWO WAYS!
        let el = &v[0];  // 1. reference to the first element. PANICS IF OUTSIDE THE VECTOR!

        let el_opt = v.get(0); // 2. getting an optional depending on the presence

        if let Some(n) = v.get(0) {
            println!("In the first position we have {n}");
        } else {
            println!("We have no element!");
        }

    }

    {
        // Remember mutable and immutable borrows in the same scope!

        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // v.push(5); // DOESN'T COMPILE! `first` is an immutable borrow of `v`, which is also
        // borrowed as mutable in the code that follows:

        println!("The first element is {first}");

        // iterating:

        for i in &mut v {  // NOTE: here `i` is a REFERENCE to the element of the array so we can do this:
            println!("i before: {i}");
            *i = *i + 50;   // the `*` is a dereference operator. To get the value in i, it is necessary
            println!("i after : {i}")
            // or `*i += 50`
        }

        println!("Vector: {:?}", v);
    }

    {
        // STRINGS: a collection type

        // We have *string slices* `str` (or in borrowed form, `&str`), built into the language
        // and the `String` of the standard library, growable and mutable, owned, UTF-8 encoded

        // a `String` is essentially a wrapper over a vector of bytes, plus commodities
        // for example we have `new`
        let a = String::new();

        // any type that have the Display trait can be converted to String using `to_string`
        let s = "hello";
        let mut s1 = s.to_string();

        // append using `push_str`
        s1.push_str(" world");

        // or format!
        s1 = format!("hello {}{}", "world", "!");

        // or with `+`
        let a = "Hello ".to_string();
        let b = "world!".to_string();
        let ab = a + &b;

        // println!("{a}"); // does NOT compile!

        // why `&b`? The signature is `fun test-workspace(self, &str) -> String`
        // For this reason, the second argument must be a string slice
        // But here &b is a &String, not &str !
        // Rust is able to do the deref coercion, turning a &String into a string slice &str
        // NOTE: `self` is not a reference, so the ownership of `a` here is moved!
        // Why is that? This achieves a much more efficient string appending system than just
        // copying around strings, since we are not really creating new strings, instead we are
        // moving them around, losing ownership of previous references.

        // LOOPING STRINGS
        // ranges and indexing are not supported by Rust strings cause of encoding reasons
        // to loop characters, use the `chars()` method.

        for (i, ch) in ab.chars().enumerate() {
            println!("Pos: {i} -> {ch}");
        }
    }


    {
        // HASH MAPS
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 5);

        let score = scores.get(&"Blue".to_string())
            .copied()
            .unwrap(); // unsafe
        println!("Getting values: {}", score);

        // iterating
        for (key, value) in &scores { // arbitrary order
            println!("{key}: {value}");
        }

        // Cool API, the `Entry` type!
        let e = scores.entry("Red".to_string()); // an Entry
        let score = e.or_insert(30); // gets the value present, or else it will insert it and return it

        // For example, counting how many times a word appears in a string
        let s = "Sopra la panca la capra campa, sotto la panca la capra crepa";
        let mut freq = HashMap::new();  // inferred in the code that follows

        for w in s.split_whitespace() {
            let c = freq.entry(w).or_insert(0);
            *c += 1; // increment the reference to the entry
        }

        println!("{freq:?}")
    }

    {
        use rand::Rng;

        // Exercise 1: Given a list of integers, use a vector and return the median (when sorted,
        // the value in the middle position) and mode (the value that occurs most often; a hash map
        // will be helpful here) of the list.

        println!("=============================\nExercise 1");
        let mut r = rand::rng();

        let mut input: Vec<i32> = (2..50)
            .map(|x| r.random_range(1..x))
            .collect();

        println!("Input is {input:?}");
        input.sort();

        let input = input;

        println!("Sorted: {input:?}");

        let median = match input.len() {
            n if n % 2 == 0 => (input[n / 2] + input[n / 2 - 1]) / 2,
            n => input[n / 2]
        };

        println!("Median is {median}");

        let mut freq = HashMap::new();

        for i in input {
            let c = freq.entry(i).or_insert(0);
            *c += 1;
        }

        let mode = freq.iter()
            .max_by_key(|(_, v)| *v)
            .expect("There is no value!")
            .1;

        println!("Mode is {mode}");
    }

    {
        // Exercise 2: Convert strings to Pig Latin. The first consonant of each word is moved to
        // the end of the word and ay is added, so first becomes irst-fay. Words that start with a
        // vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the
        // details about UTF-8 encoding!

        println!("=============================\nExercise 2");

        let vowels = ['a','e','i','o','u'];
        let is_vowel = |ch: char| -> bool {
            vowels.contains(&ch.to_ascii_lowercase())
        };

        let s = "Hello my absurd name is John".to_string();
        let mut words = vec![];

        for word in s.split_whitespace() {
            if let Some(first) = word.chars().next() {
                if is_vowel(first) {
                    words.push(format!("{}-hay", word));
                } else {
                    words.push(format!("{}-{}ay", word.split_at(first.len_utf8()).1, first));
                }
            }
        }
        println!("Result: {}", words.join(" "));
    }

    {
        // Exercise 3: Using a hash map and vectors, create a text interface to allow a user to test-workspace
        // employee names to a department in a company; for example, “Add Sally to Engineering” or
        // “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or
        // all people in the company by department, sorted alphabetically.

        println!("=============================\nExercise 3");
        let mut users: HashMap<String, Vec<String>> = HashMap::new();
        loop {
            println!("> ");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("> Failed to read");

            let words: Vec<_> = line.split_whitespace().map(|x| x.to_string()).collect();
            match words {
                w if w.is_empty() => break,
                w if &w[0].to_ascii_lowercase() == "test-workspace" => {
                    if (w.len() != 4) {
                        println!("> Too many arguments");
                        break;
                    }
                    let (name, group) = ((&w[1]).clone(), (&w[3]).clone());
                    let entry = users.entry(group.clone());

                    entry.or_insert(vec![]).push(name.clone());
                    println!("> Added {} to {}", name, group)
                }
                w if &w[0].to_ascii_lowercase() == "show" => {
                    match w.len() {
                        1 => {
                            let mut sorted_groups: Vec<_> = users.iter().collect();
                            sorted_groups.sort_by(|(g1, u1), (g2, u2)| g1.cmp(g2));
                            (&sorted_groups).into_iter().for_each(|(group, names)| {
                                let mut sorted: Vec<&String> = names.clone().iter().collect();
                                sorted.sort();
                                println!("Group: {group} -> {:?}", sorted);
                            });
                        },
                        2 => {
                            let group = (&w[1]).clone();
                            match users.get(&group) {
                                Some(u) => println!("> {u:?}"),
                                _ => println!("> Group {group} not found")
                            }
                        },
                        _ => {
                            println!("> Usage: `Show <GROUP NAME>` or `Show`");
                            break;
                        }
                    }
                }
                _ => {
                    println!("Unknown command");
                }
            }
        }
    }
}
