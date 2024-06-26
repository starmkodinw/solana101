#![feature(test)]
use core::f64;
use std::{thread, time};
extern crate test;

use hello::{
    book::Book,
    // customer::Customer,
    http::Server,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn triple(num: &mut i32) {
    *num *= 3;
}

fn hello(name: String) {
    println!("{}", name);
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

fn roman_to_integer(roman: &str) -> i32 {
    let mut result = 0;
    let mut prev_value:&i32 = &0;

    let mut roman_hashmap: HashMap<char, i32> = HashMap::new();
    roman_hashmap.insert('I', 1);
    roman_hashmap.insert('V', 5);
    roman_hashmap.insert('X', 10);
    roman_hashmap.insert('L', 50);
    roman_hashmap.insert('C', 100);
    roman_hashmap.insert('D', 500);
    roman_hashmap.insert('M', 1000);

    for c in roman.chars().rev() {
        println!("{}", c);
        let value = roman_hashmap.get(&c).unwrap();

        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }

        prev_value = value;
    }

    result
}

fn main() {
    let mut result:String = String::from("");
    let strs = ["flower","flow","flight"];
    // find longest str
    let mut max_string:&str = "";
    let mut max:i32 = 0;
    let mut str_hashmap: HashMap<&str, &str> = HashMap::new();
    for s in strs.iter() {
        str_hashmap.insert(s, s);
        if s.len() as i32 > max {
            max_string = s;
            max = s.len() as i32;
        }
    }
    str_hashmap.remove(max_string); // don't check it's self

    //loop max_string
    for (index, i) in max_string.chars().enumerate() {
        let mut is_match = true;
        str_hashmap.iter().for_each(|(k, _)| {
            let _char = k.chars().nth(index);
            if _char.is_none() { //check if index out of range
                is_match = false;
                return;
            }
            if _char.unwrap() != i { //check char at index
                is_match = false;
                return;
            }
        });
        if is_match { //if all str match add to result
            result.push(i);
        } else {
            break;
        }
    }
    println!("{} {} : result = {}", max_string, max, result);


    let roman_numeral = "XLII";
    let integer = roman_to_integer(roman_numeral);
    println!("Roman Numeral: {}, Integer: {}", roman_numeral, integer);

    let mut result: bool = true;
    let x: i32 = 2;
    let xf: f64 = x as f64;
    let place = xf.log10().floor() as i32 + 1; // Ex. 121 place = 3, 10 place = 2, 5 place = 1
    if x < 0 {
        // can't be palindrome
        result = false;
    } else {
        if place == 1 {
            // always palindrome
            result = true;
        }
        for i in 1..=place / 2 {
            let min: i64;
            let max: i64;
            if i == 1 {
                //first pair (max place & unit place)
                min = (x % 10) as i64; // unit
                max = ((x as i64 % i64::pow(10, place as u32)) / i64::pow(10, (place - 1) as u32)) as i64;
            // max place
            } else {
                // next pair (max place - i & unit place + 1)
                min = ((x % i32::pow(10, i as u32)) / i32::pow(10, (i - 1) as u32)) as i64;
                max = ((x as i64 % i64::pow(10, (place - i + 1) as u32))
                / i64::pow(10, (place - i + 1 - 1) as u32)) as i64;
            }
            if min != max {
                result = false;
                break;
            }
        }
    }

    // mod ด้วย 10**place แล้วหารด้วย 10**(place-1) จะได้เลขโดด ของแต่ละ place
    println!("{} => {} หลักหน่วย (place 1)", x , (x as i64 % i64::pow(10, (1) as u32)) / i64::pow(10, (0) as u32));
    println!("{} => {} หลักสิบ (place 2)", x , (x as i64 % i64::pow(10, (2) as u32)) / i64::pow(10, (1) as u32));
    println!("{} => {} หลักร้อย (place 3)", x , (x as i64 % i64::pow(10, (3) as u32)) / i64::pow(10, (2) as u32));
    println!("{} => {} หลักพัน (place 4)", x , (x as i64 % i64::pow(10, (4) as u32)) / i64::pow(10, (3) as u32));
    println!("result : {} {}", result, place);

    let slice = "Hello!";
    println!(
        "{} is {} bytes and also {} characters.",
        slice,
        slice.len(),
        slice.chars().count()
    );
    let slice2 = "안녕!";
    println!(
        "{} is {} bytes but only {} characters.",
        slice2,
        slice2.len(),
        slice2.chars().count()
    );

    println!(
        "The smallest i8 is {} and the biggest i8 is {}.",
        i8::MIN,
        i8::MAX
    );

    // r# makes it easier for humans to read.
    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file.");
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    );

    print_type_of("행"); //&str
    print_type_of('행'); //char

    // book.gist =========

    // {0} mean param_0.
    // {1} mean param_1.
    // # mean pretty print.
    // ? mean debug.
    let mut count = 1;

    for _i in 0..8 {
        // _i mean we won't use i
        count += 1;
    }

    println!("2. {0} = {1:#?} ----- {count}", "count", count);

    // ===================
    let server = Server::new("127.0.0.1:8000".to_string());
    if let Err(e) = server.run() {
        println!("{}", e);
    }

    let (xx1, yy1) = (10, 20); //implicit literal
    const PI: f64 = 3.14; //explicit literal
    println!("{} {} {}", xx1, yy1, PI);

    // Tuple
    let g: (i32, i32, i32) = (1, 2, 3);
    println!("{} {} {}", g.0, g.1, g.2);

    let mut x = 10;
    let y = x; // copy by value, memory address y กับ x เป็นคนละอัน
    x = 20;
    println!("x : {}, y : {}", x, y); //20 10

    let array_x = [1, 2, 3, 4, 5];
    for i in array_x.iter() {
        println!("{}", i);
    }

    let book = Book {
        name: String::from("Rust Programming"),
    };
    println!("Book name: {}", book.name);

    let s1 = String::from("abc");
    let s2 = s1; // memory s2 = s1 มันคือการ copy owner, จะใช้ s1 ไม่ได้แล้ว แต่ยังไม่ถูกทำลาย
                 // println!("{}", s1); // error : borrow of moved value: `s1`
    hello(s2); // โยน owner ไปให้ name แล้ว จะใช้ s2 อีกไม่ได้
               // println!("{}", s2); // error : borrow of moved value: `s2`

    let s3 = String::from("abcdef");
    let s4 = &s3;
    println!("{}", s4);
    // drop(s4); //ไม่สามารถ drop heap ได้ เพราะไม่ใช่ owner
    drop(s3); //drop heap ได้

    let s5 = String::from("abc");
    let s6 = s5;
    drop(s6); // drop ได้ เพราะ owner ถูกโอนมาให้ s6 แล้ว

    let mut x2 = 10;
    let y2 = &mut x2;
    *y2 = 20;
    // println!("{}", x2); //cannot borrow เพราะ y2 ยังใช้ไม่จบ => แก้โดยใช้ y2 ให้จบก่อน
    triple(y2);
    println!("{}", *y2);
    println!("{}", x2); // OK : เพราะ y2 ใช้จบแล้ว

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

    let mut x: i32 = 10;
    x = double_value(x);
    println!("x: {}", x);

    let mut numbers: &[i32] = &[62, 32, 13, 422, 56];
    numbers = &numbers[1..2]; // [32]
    println!("Numbers: {:?}", numbers);

    let mut example_string: String = String::from("Hello, example!");
    example_string = example_string.replace("example", "world");
    println!("Replaced String: {}", example_string);

    let age: i32 = 25;

    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }

    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
        if count == 3 {
            break;
        }
    }

    let number = 5;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other number"),
    }

    let student = Student::new(String::from("John Doe"), 18, 12);
    println!("Student name: {}", student.get_name());
    println!("Student age: {}", student.get_age());
    println!("Student grade: {}", student.get_grade());

    let mut student2 = Student::new(String::from("Jane Smith"), 17, 11);
    println!("Student2 name: {}", student2.get_name());
    println!("Student2 age: {}", student2.get_age());
    println!("Student2 grade: {}", student2.get_grade());

    student2.set_grade(12);
    println!("Student2 grade after update: {}", student2.get_grade());

    student.example_method();
    student2.example_method();

    #[derive(Debug)]
    enum ExampleEnum {
        Option1,
        Option2,
    }

    let example_enum = ExampleEnum::Option1;
    let example_enum2 = ExampleEnum::Option2;
    println!("Example enum: {:?}", example_enum);
    println!("Example enum2: {:?}", example_enum2);

    let mut example_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("First element: {}", example_vector[0]);
    println!("Last element: {}", example_vector[example_vector.len() - 1]);
    println!("Length: {}", example_vector.len());
    println!("Is empty: {}", example_vector.is_empty());
    println!("Contains 3: {}", example_vector.contains(&3));
    // Pushing elements to the vector
    example_vector.push(6);
    example_vector.push(7);
    println!("Updated Vector: {:?}", example_vector);

    // Removing elements from the vector
    example_vector.remove(2);
    println!(
        "Updated Vector after removing element at index 2: {:?}",
        example_vector
    );

    let mut example_hashmap: HashMap<&str, i32> = HashMap::new();
    example_hashmap.insert("key1", 10);
    example_hashmap.insert("key2", 20);
    example_hashmap.insert("key3", 30);
    example_hashmap.remove("key1");

    println!("Example Hashmap: {:?}", example_hashmap);
    println!("Value for key2: {}", example_hashmap.get("key2").unwrap());
    println!("Is key3 present: {}", example_hashmap.contains_key("key3"));
    println!("Length: {}", example_hashmap.len());
    println!("Is empty: {}", example_hashmap.is_empty());

    let result: Option<f64> = divide(10, 2);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by 0"),
    }

    let result2: Option<f64> = divide(10, 0);
    match result2 {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by 0"),
    }

    let result3: Result<f64, MyError> = divide2(10, 2);
    match result3 {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {:?}", error),
    }

    let result4: Result<f64, MyError> = divide2(10, 0);
    match result4 {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {:?}", error),
    }

    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);

    // Deserialize from JSON
    let person: Person = serde_json::from_str(&json).unwrap();
    println!("{:#?}", person);
    println!("Age: {}", person.age);

    // codebangkok
    let x = check_grade(-1);
    match x {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(g) => println!("{}", g),
    }

    let x = check_grade2(-1);
    match x {
        None => println!("error"),
        Some(v) => println!("{}", v),
    }

    let x = check_grade3(-1);
    match x {
        Err(e) => println!("{}", e),
        Ok(v) => println!("{}", v),
    }

    let x = check_grade3(100);
    if x.is_err() {
        return;
    }
    let y = x.unwrap();
    println!("{}", y);

    let x = check_grade3(100);
    if let Ok(v) = x {
        println!("{}", v);
    }

    let x = check_grade3(100);
    let y = match x {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };
    println!("{}", y);

    //Closures
    let x = add(10, 20);
    println!("x: {}", x);
    let x = |a, b| a + b;
    let y = x(10, 20);
    println!("y: {}", y);
    let y = cal(10, 20, x);
    println!("y: {}", y);
    let y = cal(10, 20, |a, b| a - b);
    println!("y: {}", y);
    let y = cal2(10, 20, add);
    println!("y: {}", y);

    concat_first(10);
    concat_first2(10);
    concat_first3(10);
    concat_second(10);
    concat_third(10);
    concat_fourth(10);
    triple2(10);
}

fn cal<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f: F) -> i32 {
    f(a, b)
}

fn cal2<F>(a: i32, b: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("score is not correct".to_string());
    }

    GradeResult::Value("A".to_string())
}

fn check_grade2(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }

    Some("A".to_string())
}

fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("score is not correct".to_string());
    }

    Ok("A".to_string())
}

enum GradeResult {
    Value(String),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

// None is returned if b is 0
// Some is returned if b is not 0
fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}

#[derive(Debug)]
enum MyError {
    DivisionByZero,
}

fn divide2(a: i32, b: i32) -> Result<f64, MyError> {
    if b == 0 {
        Err(MyError::DivisionByZero)
    } else {
        Ok(a as f64 / b as f64)
    }
}

pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn double_value(number: i32) -> i32 {
    number * 2
}

struct Student {
    name: String,
    age: u32,
    grade: u32,
}

impl Student {
    fn new(name: String, age: u32, grade: u32) -> Self {
        Student { name, age, grade }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }

    fn get_grade(&self) -> u32 {
        self.grade
    }

    fn set_grade(&mut self, grade: u32) {
        self.grade = grade;
    }
}

trait ExampleTrait {
    fn example_method(&self);
}

impl ExampleTrait for Student {
    fn example_method(&self) {
        println!("Example method implementation for Student {} ", self.name);
    }
}

// most trivial: iterate over nums,
// convert nums to string and push
// them to final result string.
fn concat_first(val: i64) -> String {
    let mut result = String::new();

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

fn concat_first2(val: u8) -> String {
    let mut result = String::new();

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

fn concat_first3(val: i128) -> String {
    let mut result = String::new();

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

// pre-decide the size of final result string.
fn concat_second(val: i64) -> String {
    let length: usize = val.to_string().len();
    let capacity: usize = (val - 1) as usize * length;

    let mut result = String::with_capacity(capacity);

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

// create a vector of numbers and using fold
// to generate the resultant string.
fn concat_third(val: i64) -> String {
    let mut vec: Vec<i64> = Vec::new();

    for i in 0..val {
        vec.push(i)
    }

    vec.iter().fold(String::new(), |accumulator, s| {
        format!("{}{}", accumulator, s)
    })
}

// heaplessly converting numbers into their string
fn concat_fourth(val: i64) -> String {
    use numtoa::NumToA;

    let mut buffer = [0u8; 20];
    let length: usize = val.to_string().len();
    let capacity: usize = (val - 1) as usize * length;
    let mut result = String::with_capacity(capacity);

    for i in 0..val {
        result.push_str(i.numtoa_str(10, &mut buffer));
    }

    result
}

fn triple2(num: i32) -> i32 {
    let t = time::Duration::from_nanos(50);
    thread::sleep(t);
    let result = num * 3;
    return result;
}

// discussion: https://www.reddit.com/r/learnrust/comments/bjyrgf/feedback_on_blogpost/
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn benchmark_first(b: &mut Bencher) {
        b.iter(|| concat_first(10));
        // b.iter(|| triple2(1003242340));
    }

    #[bench]
    fn benchmark_first2(b: &mut Bencher) {
        b.iter(|| concat_first2(10));
    }

    #[bench]
    fn benchmark_first3(b: &mut Bencher) {
        b.iter(|| concat_first3(10));
    }

    // #[bench]
    // fn benchmark_second(b: &mut Bencher) {
    //     b.iter(|| concat_second(1000));
    // }

    // #[bench]
    // fn benchmark_third(b: &mut Bencher) {
    //     b.iter(|| concat_third(1000));
    // }

    // #[bench]
    // fn benchmark_fourth(b: &mut Bencher) {
    //     b.iter(|| concat_fourth(1000));
    // }
}
