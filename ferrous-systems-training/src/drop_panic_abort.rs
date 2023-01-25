
struct LevelDB {
    handle: *mut str,
}

impl Drop for LevelDB {
    fn drop(&mut self) {
        println!("Dropping LevelDB");
    }
}

// panicks are normally reserved for unrecoverable errors that indicate a bug in the program
fn panicking_function() {
    panic!("gosh, don't call me!");
}

fn run() {
    
}