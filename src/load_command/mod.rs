pub mod build_version;
pub mod dysymtab;
pub mod segment64;
pub mod source_version;
pub mod symtab;
pub mod unix_thread;
pub mod uuid;

pub use self::{
    build_version::{BuildToolVersion, BuildVersionCommand},
    dysymtab::DysymtabCommand,
    segment64::{Section64, SegmentCommand64},
    source_version::SourceVersionCommand,
    symtab::SymtabCommand,
    unix_thread::UnixThreadCommand,
    uuid::UuidCommand,
};

use crate::io::{Endian, ReadExt as _};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt as _};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(SegmentCommand64, Vec<Section64>),
    Symtab(SymtabCommand),
    UnixThread(UnixThreadCommand),
    Dysymtab(DysymtabCommand),
    Uuid(UuidCommand),
    BuildVersion(BuildVersionCommand, Vec<BuildToolVersion>),
    SourceVersion(SourceVersionCommand),
    Unsupported(u32, Vec<u8>),
}

impl LoadCommand {
    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmd,
            LC::Symtab(cmd) => cmd.cmd,
            LC::UnixThread(cmd) => cmd.cmd,
            LC::Dysymtab(cmd) => cmd.cmd,
            LC::Uuid(cmd) => cmd.cmd,
            LC::BuildVersion(cmd, _) => cmd.cmd,
            LC::SourceVersion(cmd) => cmd.cmd,
            LC::Unsupported(cmd, _) => *cmd,
        }
    }

    pub fn cmd_size(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmdsize,
            LC::Symtab(cmd) => cmd.cmdsize,
            LC::UnixThread(cmd) => cmd.cmdsize,
            LC::Dysymtab(cmd) => cmd.cmdsize,
            LC::Uuid(cmd) => cmd.cmdsize,
            LC::BuildVersion(cmd, _) => cmd.cmdsize,
            LC::SourceVersion(cmd) => cmd.cmdsize,
            LC::Unsupported(_, data) => data.len() as u32 - 8,
        }
    }

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        use LoadCommand as LC;

        let cmd = read.read_u32_in(endian);

        let mut cmd_bytes = [0u8; 4];
        match endian {
            Endian::Little => (&mut cmd_bytes[..]).write_u32::<LittleEndian>(cmd).unwrap(),
            Endian::Big => (&mut cmd_bytes[..]).write_u32::<BigEndian>(cmd).unwrap(),
        };

        let mut read = cmd_bytes.chain(read);

        match cmd {
            SegmentCommand64::TYPE => {
                let cmd = SegmentCommand64::read_from_in(&mut read, endian);

                let mut sections = Vec::with_capacity(cmd.nsects as usize);
                for _ in 0..cmd.nsects {
                    sections.push(Section64::read_from_in(&mut read, endian));
                }

                LC::Segment64(cmd, sections)
            }
            SymtabCommand::TYPE => {
                let cmd = SymtabCommand::read_from_in(&mut read, endian);
                LC::Symtab(cmd)
            }
            UnixThreadCommand::TYPE => {
                let cmd = UnixThreadCommand::read_from_in(&mut read, endian);
                LC::UnixThread(cmd)
            }
            DysymtabCommand::TYPE => {
                let cmd = DysymtabCommand::read_from_in(&mut read, endian);
                LC::Dysymtab(cmd)
            }
            UuidCommand::TYPE => {
                let cmd = UuidCommand::read_from_in(&mut read, endian);
                LC::Uuid(cmd)
            }
            BuildVersionCommand::TYPE => {
                let cmd = BuildVersionCommand::read_from_in(&mut read, endian);

                let mut tools = Vec::with_capacity(cmd.ntools as usize);
                for _ in 0..cmd.ntools {
                    tools.push(BuildToolVersion::read_from_in(&mut read, endian));
                }
                LC::BuildVersion(cmd, tools)
            }
            SourceVersionCommand::TYPE => {
                let cmd = SourceVersionCommand::read_from_in(&mut read, endian);
                LC::SourceVersion(cmd)
            }
            _ => {
                let _cmd = read.read_u32_in(endian);
                let cmdsize = read.read_u32_in(endian) as usize;
                let mut data = Vec::with_capacity(cmdsize - 8);
                data.resize(cmdsize - 8, 0);
                read.read_exact(&mut data).unwrap();
                LC::Unsupported(cmd, data)
            }
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, sections) => {
                cmd.write_into(write);
                for section in sections.iter() {
                    section.write_into(write);
                }
            }
            LC::Symtab(cmd) => {
                cmd.write_into(write);
            }
            LC::UnixThread(cmd) => {
                cmd.write_into(write);
            }
            LC::Dysymtab(cmd) => {
                cmd.write_into(write);
            }
            LC::Uuid(cmd) => {
                cmd.write_into(write);
            }
            LC::BuildVersion(cmd, tools) => {
                cmd.write_into(write);
                for tool in tools.iter() {
                    tool.write_into(write);
                }
            }
            LC::SourceVersion(cmd) => {
                cmd.write_into(write);
            }
            LC::Unsupported(_, _) => {
                panic!("Unsupported LoadCommand is unwritable");
            }
        }
    }
}
