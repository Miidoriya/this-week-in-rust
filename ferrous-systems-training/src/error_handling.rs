use std::{error::Error, fs::File, io};

fn this_can_fail(succeeds: bool) -> Result<String, String> {
    if succeeds {
        Ok(String::from("success"))
    } else {
        Err(String::from("failure"))
    }
}

fn multiple_possible_failures() -> Result<String, String> {
    this_can_fail(true)?;
    println!("After 1st potential error.");
    this_can_fail(false)?;
    println!("After 2nd potential error.");
    Ok(String::from("All done."))
}

fn maybe_dangerous() -> Result<(), io::Error> {
    match File::open("cargo.toml") {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn run_main_error_handling() -> Result<(), Box<dyn Error>> {
    maybe_dangerous()?;
    Ok(())
}

fn dyn_errors() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("broken")?;

    let x: i32 = "ABC".parse()?;

    Ok(())
}

pub fn run() {
    let outcome = this_can_fail(true);
    println!("{:?}", outcome);

    match this_can_fail(false) {
        Ok(s) => println!("success: {}", s),
        Err(e) => println!("error: {}", e),
    }

    if this_can_fail(false).is_ok() {
        println!("It works!");
    } else {
        println!("It didn't work!");
    }

    multiple_possible_failures();

    let some_result = this_can_fail(true);
    // Only do if `some_result` is an `Ok` Variant
    let mapped_result = some_result.map(|s| s.len());
    println!("{:?}", mapped_result);
}
