mod hex;
mod macho;
mod header;

use atom_macho::header::Header64;
use clap::Parser;
use std::io::{Cursor, Read as _};
use std::fs::File;

#[derive(Parser)]
struct Args {
    file: std::path::PathBuf,
    #[clap(short, long)]
    header: bool,
}

fn main() {
    let args = Args::parse();

    let mut buf = {
        let mut file = File::open(args.file).expect("file path is invalid");
        let mut vec = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        Cursor::new(vec)
    };

    // parse header
    let header = Header64::read_from(&mut buf);
    if args.header {
        header::print_header(header);
        return;
    }
}
