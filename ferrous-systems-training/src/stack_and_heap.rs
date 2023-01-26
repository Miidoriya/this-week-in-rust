#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: &Point) {
    println!("{:?}", p);
}

pub fn run() {
    // stack allocation
    let point = Point { x: 10, y: 20 };

    // box (heap) allocation
    // represented by the type `Box<T>`
    let point_on_heap = Box::new(point);

    // box is owned but you can borrow the value
    print_point(&point_on_heap);
}