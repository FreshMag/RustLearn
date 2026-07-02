use std::fmt::{format, Debug, Display, Formatter};
use std::io::Error;
use std::iter::Sum;

// generic function (note:: PartialOrd is necessary in order to use the `>` operator
fn largest<T: PartialOrd>(elements: &[T]) -> &T {
    let mut largest = &elements[0];
    for el in elements {
        if largest < el {
            largest = el;
        }
    }
    largest
}

// generic structs
#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y
}

// generic enums
enum Result<T, E> {
    Ok(T),
    Error(E)
}

impl <T: Debug, E> Result<T, E> {
    fn if_not_error_print(&self) {
        match self {
            Result::Ok(v) => println!("Not error yay! {:?}", v),
            Result::Error(_) => {}
        }
    }
}

// I can implement methods only for specific concrete types
impl Point<f32, f32> {
    fn divide_2(&self) -> Point<f32, f32> {
        Point {
            x: self.x / 2.0,
            y: self.y / 2.0,
        }
    }
}

// ...................
// TRAITS YEAH
// ...................
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Book {
    name: String,
    description: String
}

// And then I can implement that trait
impl Summary for Book {
    fn summarize(&self) -> String {
        format!("{} and {}", self.name, self.description)
    }
}

// NOTE: I can implement trait also for other, already existing types. One between the trait and
// the type that implements it (or both) must be of the current crate (to avoid breaking code)
impl Summary for String {
    fn summarize(&self) -> String {
        self.clone()
    }
}

// We can also leave Default Implementations (much like default methods of interfaces)
pub trait Packable {
    fn pack(&self) -> String {
        println!("This is packable!");
        self.actual_pack()
    }

    fn actual_pack(&self) -> String;
}

impl Packable for Book {
    fn actual_pack(&self) -> String {
        format!("this is a cool book! Remember the title: {}", self.name)
    }
}

// We can then use Traits as parameters! This means that we accept implementations of Packable
fn notify(s: &impl Packable) {
    println!("New notification: {}", s.pack())
}

// This is the same as doing...
fn notify2<T: Packable>(s: T) {
    println!("New notification: {}", s.pack())
}

// ...but allows us to use the same type for multiple parameters
fn notify3<T: Packable>(s1: T, s2: T) {
    println!("New notification: {} and {}", s1.pack(), s2.pack())
}

// I can also specify multiple `bounds` for traits that need to be implemented
fn notify4<T: Packable + Summary>(s: T) {
    println!("{} + {}", s.pack(), s.summarize())
}

// If things starts to become too cumbersome when we have too much bounds...
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 0 }

// ...Rust allows for another, much more readable syntax: the `where` one:
fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// More realistic example using Pair struct

struct Pair<T> {
    l: T,
    r: T
}

impl<T> Pair<T> {
    fn new(l: T, r: T) -> Self {
        Self { l, r }
    }
}

impl<T> Pair<T>
where
    T: Display + PartialOrd
{
    fn cmp_print(&self) {
        if self.l > self.r {
            println!("L({}) is greater than R({})", self.l, self.r)
        } else {
            println!("R({}) is greater than L({})", self.r, self.l);
        }
    }
}

// SUUUPER FIGO!!! `Blanket implementation`
// with this line we are essentially implementing the Trait `Packable` for EVERY type that implements
// the `ToString` trait
impl<T: ToString> Packable for T {
    fn actual_pack(&self) -> String {
        format!("this is packable! {}", self.to_string())
    }
}

fn main() {
    let list1 = vec![1, 2, 5, 1000];
    let list2 = vec!['a', 'b', 'c'];
    println!("{}", largest(&list1));
    println!("{}", largest(&list2));

    let p1 = Point{x:1, y:2.0};
    let r: Result<&str, Error> = Result::Ok(" ciao");
    r.if_not_error_print();

    // p1.divide_2(); // doesn't work
    println!("{:?}", Point{x: 1.0, y: 2.0}.divide_2());

    let b = Book {
        name: String::from("Hunger Games"),
        description: String::from("This is a cool book")
    };

    println!("{}", b.summarize());
    println!("{}", b.pack());

    let p = Pair::new(1, 2);
    p.cmp_print()

}
