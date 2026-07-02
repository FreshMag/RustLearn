#![allow(dead_code, unused_variables)]

fn main() {
    {
        let s = "hello world";
        // here s is valid until the end of this block
        println!("{}", s);
    } // s is not valid here, it has been dropped since the "ownership" of the string is only within the block

    let mut s = String::from("hello"); // allocated on the heap, can be of any size
    // this string is mutable since we declared it with `mut`, we can change its value
    s.push_str(", world!");

    // how to free the memory without a Garbage Collector? Rust frees it automatically when it goes out of scope.
    // this pattern is called RAII (Resource Acquisition Is Initialization), where resources are acquired and released in a predictable manner.

    // Nasty cases
    let x = 5;
    let y = x; // this is a copy, since i32 is allocated on the stack

    let s1 = String::from("hello");
    let s2 = s1; // this copies only THE POINTER, not the data on the heap.

    // This would be a problem since when exiting the scope, the program would try to free the same memory twice 
    // (double free error)

    // To avoid this, s1 is marked as "INVALID" after the assignment, and can no longer be used.
    // println!("s1: {}", s1); // compile error: use of moved value: `s1`
    // This is called MOVING a value (similar to the concept of Shallow copy).

    let mut s = String::from("hello");
    s = String::from("ahoy"); // the previous instance of String is dropped here, to free memory.

    s2.clone(); // this creates a deep copy of the string, allocating new memory on the heap and copying the data.

    // Why some types are copied and some are moved? Depends if the type implements the Copy trait or the Drop trait.
    // they cannot implement both, since Copy types are implicitly copied and do not have a destructor, while Drop
    // types have a destructor and cannot be copied. 

    // The same behavior of assignment applies to function parameters and return values.
    // FOR EXAMPLE:
    let s = String::from("hello");
    takes_ownership(s); // s is moved into the function, and is no longer valid

    // println!("{}", s); // compile error: use of moved value: `s`. s has already been dropped after the function 
    //call, since it was moved into the function.
    {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    
        let s2 = String::from("hello"); // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2); // s2 is moved into the function, and then returned back to s3

        // println!("{s2}"); does NOT compile. s2 has been moved and it's ownership has been taken by s3.

    } // here, s3 goes out of scope and is dropped, then s2 goes out of scope but it has already been moved, so nothing 
      // happens, then s1 goes out of scope and is dropped.

    // We can use REFERENCES to avoid moving ownership!
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // we pass a reference
        println!("The length of '{}' is {}.", s1, len); // we can still use s1 here, since we only passed a reference 
                                                        // to it
        // This is called BORROWING, since we are borrowing the value of s1 without taking ownership of it.
    }

    // Strings Slices
    let mut s = String::from("hello, world");
    let slice = &s[0..5]; // this is a string slice, which is a reference to a portion of a string
    println!("slice: {}", slice);
    // Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index 
    // minus starting_index. So, in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the byte at 
    // index 6 of s with a length value of 5.

    let another_slice = &s[..2]; // without start

    let another_slice2 = &s[1..]; // like [1..len]

    let another_slice3 = &s[..]; // everything


    // Why we use &str instead of &String?
    // Because this type makes use of an advanced feature of Rust called dereferenced coercions,
    // that allows us to use both literal strings like "hello world" but also references to Strings
    // or slices. BTW, string literals *are* string SLICES.
    //
    // let s = String::from("hello world");
    // first_word(&s);
    //
    // this works because `&s` is essentially a slice of the whole String.
    // fn main() {
    //     let my_string = String::from("hello world");
    //
    //     // `first_word` works on slices of `String`s, whether partial or whole.
    //     let word = first_word(&my_string[0..6]);
    //     let word = first_word(&my_string[..]);
    //     // `first_word` also works on references to `String`s, which are equivalent
    //     // to whole slices of `String`s.
    //     let word = first_word(&my_string);
    //
    //     let my_string_literal = "hello world";
    //
    //     // `first_word` works on slices of string literals, whether partial or
    //     // whole.
    //     let word = first_word(&my_string_literal[0..6]);
    //     let word = first_word(&my_string_literal[..]);
    //
    //     // Because string literals *are* string slices already,
    //     // this works too, without the slice syntax!
    //     let word = first_word(my_string_literal);
    // }
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i-1];
            }
        }
        &s[..]
    }

    println!("first word is: {}", first_word(&s));

    println!("another slice: {another_slice}");

    // These slices are immutable borrowed references, so we cannot modify the original string through them.
    s.clear(); 
    // println!("slice: {}", slice); // compile error: cannot borrow `s` as mutable because it is also borrowed as immutable

    // There are other types of Slices, too!
    let a = [1, 2, 3, 4, 5];
    let my_slice = &a[1..3];  // this slice has type &[i32]
    assert_eq!(&my_slice, &[2, 3]);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped here

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // this moves the string out of the function and into the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string + " " // this moves the string back to the caller
}

fn calculate_length(s: &String) -> usize {
    s.len() // we can use the reference to access the string, but we do not take ownership of it
}