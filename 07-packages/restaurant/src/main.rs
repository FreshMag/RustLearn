#![allow(dead_code, unused_imports)]

// restaurant is the name of the crate. We can use `lists` thanks to the `pub use` written in lib.rs
use restaurant::lists;
// this is the same thing
use restaurant::util::lists as list2;

fn main() {
    lists::add()
}