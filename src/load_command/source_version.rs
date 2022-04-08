use crate::io::{Endian, ReadExt as _, WriteExt as _};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceVersionCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub version: Version,
}

impl SourceVersionCommand {
    pub const TYPE: u32 = 0x2a;

    pub const SIZE: u32 = 0x10;

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let cmd = read.read_u32_in(endian);
        assert_eq!(cmd, Self::TYPE);

        let cmdsize = read.read_u32_in(endian);
        assert_eq!(cmdsize, Self::SIZE);

        let version = Version(read.read_u64_in(endian));

        SourceVersionCommand {
            cmd,
            cmdsize,
            version,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.cmd);
        write.write_u32_native(self.cmdsize);
        write.write_u64_native(self.version.0);
    }
}

/// A.B.C.D.E packed as a24.b10.c10.d10.e10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(u64);

impl Version {
    /// 40 ~ 64bit
    pub fn a(&self) -> u64 {
        self.0 & 0xFFFF_FF00_0000_0000
    }

    /// 30 ~ 39bit
    pub fn b(&self) -> u64 {
        self.0 & 0x0000_00FF_C000_0000
    }

    /// 20 ~ 29bit
    pub fn c(&self) -> u64 {
        self.0 & 0x0000_0000_3FF0_0000
    }

    /// 10 ~ 19bit
    pub fn d(&self) -> u64 {
        self.0 & 0x0000_0000_000F_FC00
    }

    /// 0 ~ 9bit
    pub fn e(&self) -> u64 {
        self.0 & 0x0000_0000_0000_03FF
    }
}
