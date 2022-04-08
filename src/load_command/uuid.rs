use crate::io::{Endian, ReadExt as _, WriteExt as _};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UuidCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub uuid: [u8; 16],
}

impl UuidCommand {
    pub const TYPE: u32 = 0x1b;

    pub const SIZE: u32 = 0x18; // 24

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let cmd = read.read_u32_in(endian);
        assert_eq!(cmd, Self::TYPE);

        let cmdsize = read.read_u32_in(endian);
        assert_eq!(cmdsize, Self::SIZE);

        let mut uuid = [0; 16];
        read.read_exact(&mut uuid).unwrap();

        UuidCommand {
            cmd,
            cmdsize,
            uuid,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.cmd);
        write.write_u32_native(self.cmdsize);
        write.write_all(&self.uuid).unwrap();
    }
}
