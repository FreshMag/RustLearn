#![allow(dead_code, unused_variables)]

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn _build_user(email: String, username: String) -> User {
    // Figata: field init shorthand syntax. Since the parameter names are the same as the struct field names, we can use this shorthand syntax to initialize the struct fields.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs. Behave as named tuples. Useful when you want to give a tuple a name and make the tuple fields more meaningful.
struct ColorRGB(u8, u8, u8);

// Unit-like structs. They behave as Unit type `()`. Useful when you need to implement a trait on some type but don't have any data that you want to store in the type.
struct None;

fn main() {
    let user1 = User {
        username: String::from("Myself"),
        email: String::from("myself@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {}", user1.username);

    // cannot do user1.username = String::from("Myself2"); // error: cannot assign to `user1.username`, as `user1` is not declared as mutable

    let mut user2 = User {
        username: String::from("Myself"),
        email: String::from("myself@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    user2.username = String::from("Myself2");
    println!("User: {}", user2.username);

    // Update syntax! 
    let _user3 = User {
        email: String::from("superemail@gmail.com"), 
        ..user1 // WATCH IT! `user1.username` is moved to `user3.username`, so we cannot use `user1` anymore after this point.
    };

    let black = ColorRGB(0, 0, 0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);

    let ColorRGB(r, g, b) = black; // destructuring is done by specifying the struct name.
    println!("Red: {}, Green: {}, Blue: {}", r, g, b);

    let _result = None; // Unit-like struct can be instantiated without any fields.

    // SPOILER: it is possible to store references in structs, but it requires the use of lifetimes.
    // We will learn about lifetimes in Chapter 10. For now we will stick to storing owned data in structs.

    /* struct User {
        active: bool,
        username: &str,
        email: &str,
        sign_in_count: u64,
    }
    ...
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    }; */

    // Example program using structs

    let rect1 = (30, 50);

    fn area(r: (u32, u32)) -> u32 {
        r.0 * r.1
    }
    println!("The area of the rectangle is {}", area(rect1));

    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect2 = Rectangle {
        width: 10,
        height: 30
    };

    fn area_r(rect: &Rectangle) -> u32 {
        rect.width * rect.height  // NOTE: this does not move ownership of rect fields
    }
    println!("The area of the rectangle is {}", area_r(&rect2));

    // THIS DOES NOT COMPILE! Rectangle doesn't have the Debug trait so it cannot be printed
    // with the `:?` debug formatter
    // println!("Rect is {rect2:?}")

    // To do so, we let RectanglePrintable derive that trait

    #[derive(Debug)]
    struct RectangleD {
        width: u32,
        height: u32,
    }

    // METHODS!!!
    impl RectangleD {
        fn area(&self) -> u32 {         // &self is short for `self: &Self`. Self type is an alias for
            self.height * self.width    // the type we are implementing
        }
    }

    let rect3 = RectangleD {
        width: 30,
        height: 20,
    };

    println!("Rect is {rect3:?}");

    // Using :#? will put the values of the fields on newline
    println!("Rect is {rect3:#?}");

    // Another way is to use the `dbg!` macro, which takes ownership (and then returns it) and
    // prints also the point of the code where it has been called. It uses the standard error.
    let scale = 2;
    let rect4 = RectangleD {
        width: dbg!(30 * scale),
        height: 10,
    };

    dbg!(&rect4); // here we don't want it to take ownership of rect4, so we pass a reference.

    // METHODS!!!

    let rect5 = RectangleD {
        ..rect4
    };

    println!("So cool, this is a method: {}", rect5.area());

    // Differently from C++, we don't have the -> operator (equivalent to doing (*obj).method())
    // Rust automatically understands how that obj needs to be accessed to call that method.
    // This is called *automatic referencing and dereferencing*.
    // These are the same:
    //
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    struct RichRect {
        width: u32,
        height: u32
    }

    impl RichRect {
        fn can_hold(&self, other: &RichRect) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    impl RichRect {  // I can have multiple `impl` blocks!

        fn square(size: u32) -> Self {  // associated function (i.e. static method). Accessible with `::`
            Self {
                width: size,
                height: size
            }
        }
    }

    let r1 = RichRect { width: 5, height: 5 };
    let r2 = RichRect { width: 10, height: 7 };
    println!("r1 can hold r2: {}", r1.can_hold(&r2));
    println!("r2 can hold r1: {}", r2.can_hold(&r1));

    let r3 = RichRect::square(50);
    println!("r3 is bigger: {}", r3.can_hold(&r2));
}
