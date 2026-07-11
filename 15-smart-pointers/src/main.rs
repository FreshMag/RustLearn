use std::ops::Deref;

fn main() {
    // Box is a kind of smart pointer used for storing data on the heap


    let b = Box::new(5);  // this stores an i32 on the heap
    println!("b = {}", b); // but this is not really useful, since allocating an i32 on the stack is more efficient

    // Let's implement recursive types, as with the `cons` Lisp list pattern
    // (1, (2, (3, Nil)))

    // If we try to define something like this:
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    // This fails because Rust does not figure out how much space that data-structure will require in memory
    // Essentially, it recursively calculates the dimension of type `List`, understanding it to have "infinite" size

    // For this reason, a `Box` pointer to the heap is necessary. Since it essentially occupies the size
    // of a pointer (an i32 essentially), the `List` appears to have fixed size

    enum List<T> {
        Cons(T, Box<List<T>>),  // here, `Box` performs an *indirection*, meaning it does not store the value directly
        Nil,
    }
    use List::{Cons, Nil};

    let _l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    fn cons(v: i32, tail: Box<List<i32>>) -> Box<List<i32>> {
        Box::new(Cons(v, tail))
    }

    fn nil() -> Box<List<i32>> {
        Box::new(Nil)
    }

    let _l2 = cons(1, cons(2, cons(3, nil())));

    // Box<T> implements two Trait that enables Rust to create Smart Pointers:
    // - `Deref` : allows values like Box<T> to be treated like references
    // - `Drop`  : allows actions to be performed when the value goes out of scope (like freeing the heap when Box<T> goes out of scope)

    // --------------------------------------------------------------------------------

    // The Dereference operator *

    let x = 5;
    let y = &x;

    assert_eq!(*y, 5);

    // We can rewrite this using Box

    let x = 5;
    let y = Box::new(x);

    // the main difference here is that we are copying x value to the heap and referencing that value instead of the actual x
    // the Deref trait allows us to use the dereference operator * just like we would on normal references
    assert_eq!(*y, 5);

    // Let's define a Box<T> inspired smart pointer. This one, however, will not store values on the heap

    struct MyBox<T>(T);

    impl <T> MyBox<T> {
        fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    // assert_eq!(5, *y);  // this does not work yet

    impl <T> Deref for MyBox<T> {
        type Target = T;  // this is an associated type. Don't worry about this for now
        fn deref(&self) -> &Self::Target {
            &self.0 // the first element of the MyBox struct (in other words, the inner T value)
        }
    }

    // now, if we call *y, the compiler learned to actually call *(y.deref())
    assert_eq!(5, *y);

    // --------------------------------------------------------------------------------

    // Deref coercion

    // Rust is able to auto dereference pointers if those implement the Deref trait
    // This is the reason why &String is converted to &str at compile time

    // for example
    fn hello(m: &str) {
        println!("{m}")
    }

    let m = MyBox::new(String::from("Hello!"));

    // What is happening here?
    // Essentially Rust understands that the supplied argument does not match the signature
    // Then what it does is repeating to call `deref()` until it can.
    // So this becomes *(*(m.deref()).deref())
    hello(&m);

    // if Deref coercion wouldn't be there, we would need to write cumbersome, less readable code like:

    hello(&(*m)[..]);

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    //
    //     From &T to &U when T: Deref<Target=U>
    //     From &mut T to &mut U when T: DerefMut<Target=U>
    //     From &mut T to &U when T: Deref<Target=U>
}
