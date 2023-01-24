use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
struct Dot {
    x: i32,
    y: i32,
}

fn pacman(dot: Dot) {
    println!("Eating {:?}", dot);
}


fn write_and_close(mut file: File) -> io::Result<()> {
    file.write_all(b"Hello worlds!")
}

fn print_filelen(file: &File) -> io::Result<()> {
    let metadata = file.metadata()?;
    println!("File length: {}", metadata.len());
    Ok(())
}

fn write_to_file(file: &mut File) -> io::Result<()> {
    file.write_all(b"Hello worlds!")
}

pub fn run() -> io::Result<()> {
    let file_create = File::create("test");
    let file_create_immut = File::create("test");

    let mut file = match file_create {
        Ok(f) => f,
        Err(e) => panic!("File create failed: {}", e)
    };

    let file_immut = match file_create_immut {
        Ok(f) => f,
        Err(e) => panic!("File create failed: {}", e)
    };

    let dot = Dot { x: 10, y: 20 };
    pacman(dot);
    pacman(dot);

    print_filelen(&file)?;
    file.write_all(b"Hello worlds!")?;
    write_to_file(&mut file);
    print_filelen(&file)?;
    write_and_close(file_immut)
    // write_and_close(file_immut) // Illegal as use of moved value
}