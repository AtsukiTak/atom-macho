use mach_object::{LoadCommand, MachCommand, OFile, CPU_TYPE_X86_64};
use std::{
    fs::File,
    io::{Cursor, Read},
};

fn main() {
    let mut file = get_file();
    let mut buf = Vec::new();
    let size = file.read_to_end(&mut buf).unwrap();
    let mut cur = Cursor::new(&buf[..size]);

    if let OFile::MachFile {
        ref header,
        ref commands,
    } = OFile::parse(&mut cur).unwrap()
    {
        dbg!(header);
        dbg!(commands);
        assert_eq!(header.cputype, CPU_TYPE_X86_64);
        assert_eq!(header.ncmds as usize, commands.len());
        for &MachCommand(ref cmd, _cmdsize) in commands {
            if let &LoadCommand::SegmentCommand64 {
                ref segname,
                ref sections,
                ..
            } = cmd
            {
                println!("segment: {}", segname);
                for ref sect in sections {
                    println!("  section: {}", sect.sectname);
                }
            }
        }
    }
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
