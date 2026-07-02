

// Constants can be declared in any scope, including the global one.
// Constants are always immutable, and their type must be annotated. The expression must be constant.
const MY_AGE: u32 = 25;

fn main() {
    let x = 10;
    println!("The value of x is: {}", x);

    // you can change the immutability
    let mut x = 20;
    x += 5;
    println!("The value of x is: {}", x);

    println!("My age is: {}", MY_AGE);

    // I can shadow variables by declaring a new variable with the same name
    // I can even create scopes!
    let x = 30; // This shadows the previous x
    println!("The value of x is: {}", x);

    {
        let x = 40; // This shadows the previous x within this block
        println!("The value of x is: {}", x);
    }

    println!("After the block, the value of x is: {}", x);

    // Shadowing allows us to even change the type of a variable
    let mut spaces = "   "; // spaces is a string slice
    // we then append some more spaces to it
    println!("The spaces string is: '{}'", spaces);
    spaces = "     "; // Now spaces is a different string slice
    let spaces = spaces.len(); // Now spaces is an integer
    println!("The number of spaces is: {}", spaces);


    // floats are f32 and f64, with f64 being the default
    let x = 2.5; // f64 by default
    let y: f32 = 3.5; // explicitly f32
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // Booleans are of type bool
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    // Characters are of type char and are 4 bytes in size and can represent a Unicode scalar value
    let letter: char = 'R';
    println!("The letter is: {}", letter);

    // We also have Tuples, YAY!
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (x, y, z) = tup; // destruct
    println!("The value of x is: {}", x);
    // we can also access tuple elements directly
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);
    // the () is an empty tuple, also known as the unit type, and it represents an empty value or a value that has no meaningful data
    let unit: () = ();

    // And then we have Arrays, which are fixed in size and can only contain elements of the same type
    let arr = [1, 2, 3, 4, 5]; // allocated on the stack
    // we can also specify the type and size of the array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first element of the array is: {}", arr[0]);
    println!("The second element of the array is: {}", arr[1]);
    // we can also create an array with the same value repeated
    let arr = [0; 5]; // creates an array of 5 elements, all initialized to 0

    println!("If we go out of bounds, we get a panic!"); // this will cause a panic at runtime
    // println!("The sixth element of the array is: {}", arr[5]); // this will cause a panic at runtime because we are 
    // trying to access an index that is out of bounds

    // Functions
    example_function();
    fun_with_par(42);

    // statements and expressions
    // the nested blocks are expressions, for example
    let x = {
        let y = 10;
        y + 5 // this is the value of the block expression, and it will be assigned to x
    };
    println!("The value of x is: {}", x);

    let x = five(); // this will call the five function and assign its return value to x
    println!("The value of the returned x is: {}", x);

    let x = five_plus_one(10); // this will call the five_plus_one function with the argument 10 and assign its return value to x
    println!("The value of the returned x is: {}", x);

    // Control flow: IF ELSE expressions
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    let category = if MY_AGE > 18 { "adult" } else { "child" }; // this is an if expression, and it will return a value based on the condition
    println!("The category is: {}", category);

    // This does not compile: incompatible types in the if expression.
    // let error = if MY_AGE < 18 { "young" } else { 15 };
}

fn example_function() {
    println!("This is an example function!");
}

fn fun_with_par(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5 // this is the return value of the function, and it will be returned to the caller
}

fn five_plus_one(x: i32) -> i32 {
    // x + 1;  // this does not compile because it is a statement, not an expression, and it does not return a value
    x + 1 // this is an expression, and it will return the value of x
}
