//! # Top documentation
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient. This is usually included in lib.rs

///
/// This is Markdown, published into HTML!
///
/// # Examples
/// Nice example:
/// ```
/// let test = vec![1, 2];
///
/// assert_eq!(test, vec![1,2]);
/// ```
///
/// We can generate doc using `cargo doc`
///
/// Rustceans often use this sections in the documentation:
///
/// # Panics
/// These are the scenarios in which the function being documented could panic. Callers
/// of the function who don’t want their programs to panic should make sure they don’t call the
/// function in these situations.
/// # Errors
/// If the function returns a Result, describing the kinds of errors that might occur and
/// what conditions might cause those errors to be returned can be helpful to callers so that they
/// can write code to handle the different kinds of errors in different ways.
/// # Safety
/// If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be
/// a section explaining why the function is unsafe and covering the invariants that the function
/// expects callers to uphold.
fn main() {
    println!("Hello, world!");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}