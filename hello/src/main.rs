fn main() {
    println!("Hello, world!");

    let number: i32 = 42;
    let message: &str = "Hello, Rust!";
    let is_true: bool = true;
    let pi: f64 = 3.14159;

    println!("Number: {}", number);
    println!("Message: {}", message);
    println!("Is True: {}", is_true);
    println!("Pi: {}", pi);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);

    let num: i32 = 10;
    println!("Is {} even? {}", num, is_even(num));
}


pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}