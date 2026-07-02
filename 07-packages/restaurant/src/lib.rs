#![allow(dead_code, unused_imports)]

mod front_of_the_house {

    fn deliver_order() {

    }

    pub struct Breakfast {
        pub toast: String,     // public
        seasonal_fruit: String // private
    }

    pub enum Appetizer { // if an enum is public, all of its fields become public
        Soup,
        Salad
    }

    pub mod hosting {
        pub fn add_to_waitlist() {
            super::deliver_order() // super is relative for parent
        }

        fn seat_at_table() {

        }
    }
}

// import the whole module
// idiomatic way, for functions bring the parent into scope (so you know it is not a local function)
use front_of_the_house::hosting;

// for structs and enums, instead, it's idiomatic to bring the whole path into scope
use front_of_the_house::Breakfast;

mod customer {

    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist(); // this is not visible because the `use` appears in another scope
    }
}

// we can also have aliases!
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    panic!()
}

fn function2() -> IoResult<()> {
    panic!()
}

mod module {
    pub mod nested {
        pub fn my_func() {

        }
    }
}

// RE-Exporting. This makes available `hosting` from this module also, and we don't need to make the
// `module` package public!
pub use crate::module::nested;

// Multiple imports
use std::{cmp::Ordering, fmt};

// to bring both `io` and `io::Write`
use std::io::{self, Write};

// module implemented in another file
pub mod util;

pub use crate::util::lists;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();

    // relative path
    front_of_the_house::hosting::add_to_waitlist();

    // Thanks to the import of the module, I can use the functions directly
    hosting::add_to_waitlist();

    lists::add();
}
