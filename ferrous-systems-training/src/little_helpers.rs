use std::fs::File;

#[derive(Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Owned {
    string_data: String
}

fn returns_string() -> String {
    String::from("Hello World")
}

fn hello(who: &str) {
    println!("Hello {}", who);
}

#[test]
fn my_test() {
    assert_eq!(1,1);
}

#[test]
#[should_panic]
fn failing_test() {
    assert_eq!(1,2);
}

fn addition(a: i32, b: i32) -> i32 {
    todo!();
}

#[test]
fn test_addition() {
    assert_eq!(addition(1,2), 3);
}

pub fn run() {
    let p = Point { x: 10, y: 20 };
    println!("{:?}", p); // debug as string
    println!("{:#?}", p); // debug with structure

    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 10, y: 20 };
    if p1 == p2 {
        println!("p1 == p2"); // Eq allows this comparison
    }
    assert_eq!(p1, p2); // PartialEq allows this comparison

    let file: File = File::open("../cargo.toml").unwrap(); // use unwrap when you expect the result to be ok

    let slice: &str = "Hello World";
    let string: String = String::from(slice);
    returns_string();
    hello("Miidoriya")
}