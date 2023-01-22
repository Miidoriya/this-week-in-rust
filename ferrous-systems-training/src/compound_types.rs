// standard struct
struct Point {
    x: i32,
    y: i32,
}

// standard enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// enums with values
enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

// enum with Structured Variants
enum Actions {
    StickAround,
    MoveTo { x: i32, y: i32 },
}

fn prints_but_returns_nothing(data: &str) -> () {
    println!("passed string: {}", data);
}

pub fn run() {
    // example of struct construction
    let p = Point { x: 10, y: 20 };

    // example of struct field access
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);

    // tuple example
    let p = (10, 20);

    // example of tuple value access
    println!("p.0 = {}", p.0);
    println!("p.1 = {}", p.1);

    // enum usage
    // different enum choices are called `variants`
    let _direction = Direction::Left;

    // enum with value usage
    let _movement = Movement::Left(10);

    // enums with structured variants usage
    let _action = Actions::MoveTo { x: 10, y: 20 };
    
}
