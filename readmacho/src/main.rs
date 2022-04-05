mod hex;
mod macho;

use std::{
    fs::File,
    io::{Cursor, Read as _},
};

fn main() {
    let mut file = get_file();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec).unwrap();

    let mut buf = Cursor::new(vec);

    let macho = macho::read_macho(&mut buf);
    dbg!(&macho);
}

fn get_file() -> File {
    match std::env::args().skip(1).next() {
        Some(s) => File::open(s).expect("file path is invalid"),
        None => {
            println!("target file's path is required.");
            std::process::exit(1)
        }
    }
}
