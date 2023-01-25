pub fn run() {
    let vec = vec![1, 2, 3];

    // owned iterator
    let iter = vec.into_iter();

    for i in iter {
        println!("{}", i);
    }

    // println!("iter is now invalid: {:?}", iter);

    let vec = vec![1, 2, 3];

    // borrowed iterator
    let iter = vec.iter();

    for i in iter {
        println!("{}", i);
    }

    println!("iter is still valid: {:?}", vec);

    // mutably borroed iterator
    let mut vec = vec![1, 2, 3];
    let iter_mut = vec.iter_mut();

    for i in iter_mut {
        *i += 1;
    }

    print!("{:?}", vec);

    // some conventions
    // -------------------------------------------------
    //     Owned    |    Borrowed   |   Mutably Borrowed
    // -------------------------------------------------
    // into_iter()  |    .iter()    |   .iter_mut()

    // common uses: next
    let items = vec![0, 1, 2];
    let mut iterator = items.into_iter();
    println!("iterator.next() = {:?}", iterator.next());
    println!("iterator.next() = {:?}", iterator.next());
    println!("iterator.next() = {:?}", iterator.next());
    println!("iterator.next() = {:?}", iterator.next());

    // common uses: map
    let items = (0..10_000).map(|x| match x {
        x if x % 15 == 0 => String::from("FizzBuzz"),
        x if x % 3 == 0 => String::from("Fizz"),
        x if x % 5 == 0 => String::from("Buzz"),
        x => format!("{}", x),
    });
    for item in items {
        println!("{}", item);
    }

    // common uses: filter
    let evens = (0..10_000).filter(|x| x % 2 == 0);
    for item in evens {
        println!("{}", item);
    }
}
