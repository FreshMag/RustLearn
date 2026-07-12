use std::cell::RefCell;
use std::ops::Deref;

fn main() {
    // Box is a kind of smart pointer used for storing data on the heap

    let b = Box::new(5); // this stores an i32 on the heap
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
        Cons(T, Box<List<T>>), // here, `Box` performs an *indirection*, meaning it does not store the value directly
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

    impl<T> MyBox<T> {
        fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    // assert_eq!(5, *y);  // this does not work yet

    impl<T> Deref for MyBox<T> {
        type Target = T; // this is an associated type. Don't worry about this for now
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

    // --------------------------------------------------------------------------------

    // The Drop trait

    // The `Drop` trait allows us to specify a piece of code to be run when a value goes out of scope
    // This can useful to clear resources automatically to prevent leaks or load that accumulates
    // into a future crash of the system

    // Let's implement it to print a message when the object is dropped!

    struct Droppable(i32);

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Inside `drop`: The data {} is being dropped!", self.0)
        }
    }
    {
        let _a = Droppable(5);
        let _b = Droppable(10);
    }
    // this will print two messages in reverse order of their declaration (first 10, then 5)

    // Let's suppose we want to drop something *before* it goes out of scope (for example, a lock)
    // We can't call the `drop` method directly
    {
        let a = Droppable(5);
        // a.drop(); // this does not compile

        // Instead, Rust provides a special method called std::mem::drop, to free instances before they
        // go out of scope

        println!("`a` instance just created");
        println!("Calling drop");
        drop(a);
        println!("Dropped `a` before the end of the scope")
    }

    // --------------------------------------------------------------------------------

    // The Rc<T> trait. Reference counting

    // There are cases where we need multiple owners of a value. For example, in a graph: multiple
    // edges might point to the same node, and that node is owned by all the edges that goes into
    // it.

    // Reference counting allows for keeping track of the number of owners of a value, allowing Rust
    // to know when a value has no owners left. This is especially needed when we don't know, at
    // compile time, the last of the owners that will go out of scope.

    // Note: this Reference counting system only works in single-thread. Multi-threading has another
    // way to manage reference counting

    // Resuing the Cons concept from before, we will model this situation:

    // b --> [3|*] --+
    //               |
    //        a --> [5|*] ------> [10|/] --> Nil
    //               |
    // c --> [4|*] --+

    // where a sub list is shared among two lists.

    // If we try to use Box<T> here, we'll fail

    let a = cons(5, cons(10, nil()));
    let b = cons(3, a);
    // let c = cons(4, a);  // using a value after it being moved

    // We could change the definition of Cons to hold references instead, but then we would have to
    // specify lifetime parameters. By specifying lifetime parameters, we would be specifying that
    // every element in the list will live at least as long as the entire list (not true in every
    // scenario)

    // so we need Rc<T>, let's change the List struct
    use std::rc::Rc;

    #[allow(dead_code, unused)]

    enum ListG<T> {
        Cons(T, Rc<ListG<T>>),
        Nil,
    }

    let a = Rc::new(ListG::Cons(
        5,
        Rc::new(ListG::Cons(10, Rc::new(ListG::Nil))),
    ));
    // Let's see the reference counting in action!
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let b = ListG::Cons(3, Rc::clone(&a)); // Rc::clone is the convention in Rust, instead of doing a.clone().
    // This is because normally `.clone()` makes a deep copy, while
    // here we're just using it to increment the reference counting

    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = ListG::Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a))
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // --------------------------------------------------------------------------------

    // RefCell<T>

    // Rc<T> works only with immutable references.
    // By default, you can always ONLY HAVE 1 MUTABLE REFERENCE to a value.
    // Box<T> contains a singular reference, but cannot go further than 1 mutable reference.

    // RefCell<T> exists for maintaining multiple immutable references, "acting like an immutable reference".
    // This is potentially unsafe: Rust compiler enforces the "single mutable reference" to prevent
    // dangerous code that might leak or race. With RefCell we are essentially disabling these checks
    // to perform them on our own, allowing for safe memory scenarios that the compiler (which is conservative) would block.

    // Does this RefCell potentially break everything?
    // NO. The checks are still performed, but at RUNTIME. This means that in the cases where
    // memory is being treated badly:
    // - normally we would catch it at compile time, causing a compiler error
    // - HERE, instead, we catch it at *runtime*, meaning the program panics. This also means that
    //   checks cause an overhead since they are done while the program is running.

    // One place where RefCell is particularly useful, is to create Mock objects.
    // Consider a place where we want to track down a number and send messages when this number goes
    // above a limit we have configured.

    trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    // Now we need, for testing purposes, to check if messages are sent when we set a high value
    // For this purpose, we create a Mock messenger that allows us to check what messages are sent

    // THIS DOES NOT WORK!

    // struct MockMessenger {
    //     sent_messages: Vec<String>
    // }
    //
    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }
    //
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));  // this is trying to mutate itself, but the reference is immutable!
    //                                                          // Of course, we don't want to change the original signature just
    //                                                          // for testing purposes...
    //     }
    // }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));  // this becomes mutable!
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    // Note that regular Rust rules still stand. We can have only 1 mutable borrow and infinite immutable ones.

    // THIS WILL PANIC!!!
    // impl Messenger for MockMessenger {
    //         fn send(&self, message: &str) {
    //             let mut one_borrow = self.sent_messages.borrow_mut();
    //             let mut two_borrow = self.sent_messages.borrow_mut();
    //
    //             one_borrow.push(String::from(message));
    //             two_borrow.push(String::from(message));
    //         }
    //     }

    // How does this work?
    // When creating immutable and mutable references, we use the & and &mut syntax, respectively.
    // With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API
    // that belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and
    // borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can
    // treat them like regular references.
    //
    // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently
    // active. Every time we call borrow, the RefCell<T> increases its count of how many immutable
    // borrows are active. When a Ref<T> value goes out of scope, the count of immutable borrows
    // goes down by 1. Just like the compile-time borrowing rules, RefCell<T> lets us have many
    // immutable borrows or one mutable borrow at any point in time.
    //
    // If we try to violate these rules, rather than getting a compiler error as we would with
    // references, the implementation of RefCell<T> will panic at runtime.


    // We can do some crazy stuff with RefCell

    #[derive(Debug)]
    enum ListR {
        Cons(Rc<RefCell<i32>>, Rc<ListR>),
        Nil,
    }

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ListR::Cons(Rc::clone(&value), Rc::new(ListR::Nil)));

    let b = ListR::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ListR::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}"); // the value of `a` changed here too, even if we declared it as mutable!
    println!("c after = {c:?}");
}
