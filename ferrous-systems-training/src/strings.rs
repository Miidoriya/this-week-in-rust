fn accept_either<S>(thing: S) -> String
where
    S: AsRef<str>,
{
    String::from("foo") + thing.as_ref()
}

pub fn run() {
    let this: &'static str = "Hello"; // &'static str
    let that: String = String::from("World"); // String
    let other = that.as_str(); // &str

    let part_one = String::from("Hello ");
    let part_two = String::from("there ");
    let whole = part_one + &part_two + "world!";
    println!("{}", whole);

    // common string tasks: Splitting
    let words = "the quick brown fox jumps over the lazy dog";
    let each: Vec<_> = words.split_whitespace().collect();
    println!("{:?}", each);

    // common string tasks: Concatenation
    let animal = String::from("Cow");
    let sound = String::from("moo");
    let words = [&animal, " says ", &sound].concat();
    println!("{}", words);

    // common string tasks: Replacing
    let words = "Cow says moo";
    let replaced = words.replace("moo", "rawr");
    println!("{}", replaced);

    // Aceepting String or str
    println!("{}", accept_either("Hello"));
    println!("{}", accept_either(String::from("Hello")));

    // Raw string literals
    let json = r##"{"name": "John", "age": 31, "city": "New York"}"##;
    assert_eq!(json, "{\"name\": \"John\", \"age\": 31, \"city\": \"New York\"}");

    // Byte string literals
    let byte_string: &[u8] = b"allows ASCII and \xF0\x9F\x98\x80 only";
    println!("Can Debug fmt but not Display fmt: {:?}", byte_string);
    if let Ok(string) = std::str::from_utf8(byte_string) {
        println!("Can now Display: '{}'", string);
    }
}
