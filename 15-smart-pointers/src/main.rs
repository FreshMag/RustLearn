fn main() {
    // Box is a kind of smart pointer used for storing data on the heap


    let b = Box::new(5);  // this stores an i32 on the heap
    println!("b = {}", b);
}
