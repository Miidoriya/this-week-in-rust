use std::{fs::File, io::{self, Read}};

enum Direction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
}

fn going_west(dir: &Direction) -> bool {
    match dir {
        Direction::West(_) => true,
        _ => false,
    }
}

fn going_south_or_west(dir: &Direction) -> bool {
    match dir {
        Direction::South(_) | Direction::West(_) => true,
        _ => false,
    }
}

fn get_number() -> u32 {
    return 5;

    8;
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn run() {
    let a = 4;
    match a % 3 {
        0 => println!("a is divisible by 3"),
        _ => println!("a is not divisible by 3"),
    }

    let will_overflow: Option<u8> = 10_u8.checked_add(250);
    match will_overflow {
        Some(sum) => println!("interesting: {}", sum),
        None => println!("addition overflow!"),
    }

    let file_open: Result<File, io::Error> = File::open("Does not exist");
    match file_open {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Open failed: {:?}", e),
    }

    let sum: Option<u8> = 5_u8.checked_add(5);
    match sum {
        Some(sum) if sum % 2 == 0 => println!("5+5 is even!"),
        _ => println!("5+5 ... isn't even?"),
    }

    let maybe_arg = std::env::args().nth(2);
    if let Some(arg) = maybe_arg {
        println!("Got second command line arguement: {}", arg);
    }
    
    let mut i = 0;

    loop {
        i += 1;
        if i > 100 {
            break;
        }
    }

    let numbers = vec![1, 2, 3];
    for number in numbers {
        println!("{}", number);
    }

    let mut i = 0;
    while !(i > 100) {
        i += 1;
    }

    let mut iter = vec![1, 2, 3].into_iter();
    while let Some(i) = iter.next() {
        println!("number {}", i);
    }

    'outer: for i in 0..10 {
        loop {
            if i < 5 {
                continue 'outer;
            } else {
                break 'outer;
            }
        }
    }

}