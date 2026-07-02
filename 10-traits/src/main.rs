use std::fmt::Debug;
use std::io::Error;

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

fn main() {
    let list1 = vec![1, 2, 5, 1000];
    let list2 = vec!['a', 'b', 'c'];
    println!("{}", largest(&list1));
    println!("{}", largest(&list2));

    let p1 = Point{x:1, y:2.0};
    let r: Result<&str, Error> = Result::Ok(" ciao");
    r.if_not_error_print();

    // p1.divide_2(); // doesn't work
    println!("{:?}", Point{x: 1.0, y: 2.0}.divide_2())
}
