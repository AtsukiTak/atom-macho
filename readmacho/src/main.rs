mod cmds;
mod cmd;
mod header;
mod hex;
mod macho;

use atom_macho::{header::Header64, load_command::LoadCommand};
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

    // print header
    let header = Header64::read_from(&mut buf);
    if args.header {
        println!("");
        header::print_header(&header);
    }

    // print list of load commands
    let load_commands = (0..header.n_cmds)
        .map(|_| LoadCommand::read_from_in(&mut buf, header.endian()))
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
