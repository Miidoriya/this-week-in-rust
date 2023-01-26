struct Point<Precision> {
    x: Precision,
    y: Precision,
}

// generic enum
enum Either<T, X> {
    Left(T),
    Right(X),
}

fn run() {
    let point = Point { x: 1_u32, y: 2 };
    let point: Point<i32> = Point { x: 1, y: 2 };

    let alternative: Either<i32, f64> = Either::Left(123);
}

// generic functions
fn accept_any_type<T>(data: T) {
    // .....
}

fn accept_and_return_any_type<T>(data: T) -> T {
    data
}