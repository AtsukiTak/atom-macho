mod cmd;
mod cmds;
mod header;
mod hex;
mod macho;

use atom_macho::{
    header::{Header, Header64, CpuType, CpuSubTypeX86_64},
    load_command::LoadCommand,
};
use clap::Parser;
use std::fs::File;
use std::io::{Cursor, Read as _};

#[derive(Parser)]
struct Args {
    file: std::path::PathBuf,

    /// Print header
    #[clap(short = 'H')]
    header: bool,

    /// Print list of load command
    #[clap(short = 'L')]
    load_commands: bool,

    /// Print overview of the load command.
    #[clap(short = 'l', name = "LOAD_COMMAND_IDX")]
    load_command: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    let mut buf = {
        let mut file = File::open(args.file.clone()).expect("file path is invalid");
        let mut vec = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        Cursor::new(vec)
    };

    let header = Header::read_from(&mut buf);

    // TODO: detect machine cpu
    let cpu_type = CpuType::X86_64(CpuSubTypeX86_64::All);

    let mach_header = match header {
        Header::Mach(h) => h,
        Header::Fat(fat_header) => {
            if let Some(fat_arch) = fat_header.fat_archs.iter().find(|fat_arch| fat_arch.cpu_type == cpu_type) {
                buf.set_position(fat_arch.offset as u64);
                Header64::read_from(&mut buf)
            } else {
                panic!("Header for {:?} is not found", cpu_type);
            }
        }
    };

    if args.header {
        println!("");
        header::print_header(&mach_header);
    }

    // print list of load commands
    let load_commands = (0..mach_header.n_cmds)
        .map(|_| LoadCommand::read_from_in(&mut buf, mach_header.endian()))
        .collect::<Vec<LoadCommand>>();
    if args.load_commands {
        println!("");
        cmds::print_cmds(&load_commands);
    }

    // print specified load command
    for cmd_idx in args.load_command.iter() {
        println!("");
        cmd::print_cmd(&load_commands, *cmd_idx);
    }
}
