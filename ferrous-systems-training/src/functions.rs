use std::fmt::Debug;

// declaration of a function
fn add(first: i32, second: i32) -> i32 {
    first + second
}

// arguements
fn return_nothing() {}

fn return_a_random() -> i32 {
    42
}

fn mnaybe_return_a_random(should: bool) -> Option<i32> {
    if should {
        Some(42)
    } else {
        None
    }
}

// returning
fn doesnt_return() {
    true;
}

// fn doesnt_compile() -> bool {
//     true;
// }

fn returns() -> bool {
    true
}

// generic functions
fn generic<T>(thing: T) -> T {
    thing
}

// with bounds
fn print_anything<T: Debug>(thing: T) {
    println!("{:?}", thing);
}

// bounded example (Above and below are equivalent)
fn prints_anything<T>(thing: T)
where
    T: Debug,
{
    println!("{:?}", thing);
}

// functions for types
struct Square(f32);

fn square_num_sides() ->  u32 {
    4
}

fn square_calc_area(square: &Square) -> f32 {
    square.0 * square.0
}

fn square_double_size(square: &mut Square) {
    square.0 *= 2.0;
}

// rusts better solution to the above... assocuiated functions
// includes data access via self and mutating data via &mut self
impl Square {
    fn num_sides() -> u32 {
        4
    }

    fn calc_area(&self) -> f32 {
        self.0 * self.0
    }

    fn double_size(&mut self) {
        self.0 *= 2.0;
    }
    
    // methods that take ownership of data called 'destroy'
    // done via the direct use of'self' instead of '&self'
    fn destroy(self) {
        println!("destroying square");
    }

}

pub fn run() {
    let mut square = Square(10.0);
    println!("square has {} sides", Square::num_sides());
    println!("square has an area of {}", square.calc_area());
    square.double_size();
    println!("square has an area of {}", square.calc_area());
    square.destroy();
    // This line won't compile
    println!("Area: {}", square.calc_area());
}

// methods that can access data
