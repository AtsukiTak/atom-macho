use crate::io::{Endian, ReadExt as _, WriteExt as _};
use byteorder::{BigEndian, NativeEndian, ReadBytesExt as _, WriteBytesExt as _};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    fmt,
    io::{Read, Write},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header {
    Mach(Header64),
    Fat(FatHeader),
}

impl Header {
    pub fn magic(&self) -> Magic {
        match self {
            Header::Mach(mach_header) => mach_header.magic,
            Header::Fat(fat_header) => fat_header.magic,
        }
    }

    pub fn read_from<R: Read>(read: &mut R) -> Self {
        let magic_n = read.read_u32::<NativeEndian>().unwrap();
        let magic = Magic::from_u32(magic_n);

        match magic {
            Magic::Magic64 | Magic::Cigam64 => {
                Header::Mach(Header64::read_after_magic(read, magic))
            }
            Magic::Magic | Magic::Cigam => unimplemented!(),
            Magic::FatMagic | Magic::FatCigam => {
                Header::Fat(FatHeader::read_after_magic(read, magic))
            }
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        match self {
            Header::Mach(mach_header) => mach_header.write_into(write),
            Header::Fat(fat_header) => fat_header.write_into(write),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header64 {
    pub magic: Magic,
    pub cpu_type: CpuType,
    pub file_type: FileType,
    pub n_cmds: u32,
    pub size_of_cmds: u32,
    pub flags: Flags,
    pub reserved: u32,
}

impl Header64 {
    pub const SIZE: u32 = 0x20; // 32 bytes

    pub fn read_from<R: Read>(read: &mut R) -> Self {
        let magic_n = read.read_u32::<NativeEndian>().unwrap();
        let magic = Magic::from_u32(magic_n);

        Self::read_after_magic(read, magic)
    }

    fn read_after_magic<R: Read>(read: &mut R, magic: Magic) -> Self {
        assert!(magic == Magic::Magic64 || magic == Magic::Cigam64);

        let endian = magic.endian();

        let cpu_type_n = read.read_i32_in(endian);
        let cpu_subtype_n = read.read_i32_in(endian);
        let cpu_type = CpuType::from_i32_i32(cpu_type_n, cpu_subtype_n);

        let file_type_n = read.read_u32_in(endian);
        let file_type = FileType::from_u32(file_type_n);

        let n_cmds = read.read_u32_in(endian);

        let size_of_cmds = read.read_u32_in(endian);

        let flags_n = read.read_u32_in(endian);
        let flags = Flags::from_u32(flags_n);

        let reserved = read.read_u32_in(endian);

        Header64 {
            magic,
            cpu_type,
            file_type,
            n_cmds,
            size_of_cmds,
            flags,
            reserved,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.magic.to_u32());
        let (cpu_type_n, cpu_subtype_n) = self.cpu_type.to_i32_i32();
        write.write_i32_native(cpu_type_n);
        write.write_i32_native(cpu_subtype_n);
        write.write_u32_native(self.file_type.to_u32());
        write.write_u32_native(self.n_cmds);
        write.write_u32_native(self.size_of_cmds);
        write.write_u32_native(self.flags.to_u32());
        write.write_u32_native(self.reserved);
    }

    pub fn endian(&self) -> Endian {
        self.magic.endian()
    }
}

/// An integer containing a value identifying this file as a Mach-O file.
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Magic {
    /// use if the file is intended for use on a 64bit CPU with the **same** endianness as the host
    /// computer.
    Magic64 = 0xfeedfacf,
    /// use if the file is intended for use on a 64bit CPU with the **reverse** endianness as the
    /// host computer.
    Cigam64 = 0xcffaedfe,
    /// use if the file is intended for use on a 32bit CPU with the **same** endianness as the host
    /// computer.
    Magic = 0xfeedface,
    /// use if the file is intended for use on a 32bit CPU with the **reverse** endianness as the
    /// host computer.
    Cigam = 0xcefaedfe,
    /// use if the file contains code for more than one architecture and is intended for use on a
    /// CPU with the **same** endianness as the host computer.
    FatMagic = 0xcafebabe,
    /// use if the file contains code for more than one architecture and is intended for use on a
    /// CPU with the **reverse** endianness as the host computer.
    FatCigam = 0xbebafeca,
}

impl Magic {
    pub fn from_u32_checked(n: u32) -> Option<Self> {
        FromPrimitive::from_u32(n)
    }

    pub fn from_u32(n: u32) -> Self {
        Magic::from_u32_checked(n).unwrap()
    }

    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    pub fn endian(&self) -> Endian {
        match self {
            Magic::Magic64 | Magic::Magic | Magic::FatMagic => Endian::NATIVE,
            Magic::Cigam64 | Magic::Cigam | Magic::FatCigam => Endian::REVERSE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuType {
    X86(CpuSubTypeX86),
    X86_64(CpuSubTypeX86_64),
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86 {
    All = 0x3,
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86_64 {
    All = 0x3,
}

impl CpuType {
    const CPU_ARCH_ABI64: i32 = 0x01000000;
    const CPU_TYPE_X86: i32 = 0x7;
    const CPU_TYPE_X86_64: i32 = Self::CPU_TYPE_X86 | Self::CPU_ARCH_ABI64;

    pub fn from_i32_i32(cpu_type_n: i32, cpu_subtype_n: i32) -> Self {
        // x86
        if cpu_type_n == Self::CPU_TYPE_X86 {
            let cpu_subtype = CpuSubTypeX86::from_i32(cpu_subtype_n).unwrap();
            CpuType::X86(cpu_subtype)
        // x86_64
        } else if cpu_type_n == Self::CPU_TYPE_X86_64 {
            let cpu_subtype = CpuSubTypeX86_64::from_i32(cpu_subtype_n).unwrap();
            CpuType::X86_64(cpu_subtype)
        } else {
            panic!("Unsupported cpu type {}", cpu_type_n)
        }
    }

    pub fn to_i32_i32(&self) -> (i32, i32) {
        match self {
            CpuType::X86(sub) => (CpuType::CPU_TYPE_X86, *sub as i32),
            CpuType::X86_64(sub) => (CpuType::CPU_TYPE_X86_64, *sub as i32),
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
/// Declared in /usr/include/mach-o/loader.h
pub enum FileType {
    Object = 0x1,
    Execute = 0x2,
    FVMLib = 0x3,
    Core = 0x4,
    Preload = 0x5,
    Dylib = 0x6,
    Dylinker = 0x7,
    Bundle = 0x8,
    Dsym = 0xA,
}

impl FileType {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap()
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Flag {
    NoUndefs                = 0x000001,
    IncrLink                = 0x000002,
    DyldLink                = 0x000004,
    BindAtLoad              = 0x000008,
    PreBound                = 0x000010,
    SplitSegs               = 0x000020,
    TwoLevel                = 0x000080,
    ForceFlat               = 0x000100,
    NoMultiDefs             = 0x000200,
    NoFixPreBinding         = 0x000400,
    PreBindable             = 0x000800,
    AllModsBound            = 0x001000,
    SubsectionsViaSymbols   = 0x002000,
    Canonical               = 0x004000,
    Pie                     = 0x200000,
    HasTlvDescriptors       = 0x800000,
}

impl Flag {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap()
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Flags {
    flags: Vec<Flag>,
}

impl Flags {
    pub fn new() -> Flags {
        Flags { flags: Vec::new() }
    }

    pub fn push(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Flag> + 'a {
        self.flags.iter().copied()
    }

    pub fn from_u32(flags_n: u32) -> Self {
        let mut flags = Flags::new();
        for i in 0..=31 {
            let flag_n = flags_n & (1 << i);
            if flag_n != 0 {
                let flag = Flag::from_u32(flag_n);
                flags.push(flag);
            }
        }

        flags
    }

    pub fn to_u32(&self) -> u32 {
        let mut flag_n = 0u32;

        for flag in self.flags.iter() {
            flag_n |= flag.to_u32();
        }

        flag_n
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.flags.iter()).finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FatHeader {
    pub magic: Magic,
    pub fat_archs: Vec<FatArch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatArch {
    /// Specifies the CPU family.
    pub cpu_type: CpuType,
    /// Offset to the beginning of the data for this CPU.
    pub offset: u32,
    /// Size of the data for this CPU.
    pub size: u32,
    /// The power of 2 alignment for the offset of the object file for the architecture specified
    /// in `cpu_type` within the binary. This is required to ensure that, if this binary is
    /// changed, the content it retains are correctly aligned for virtual memory paging and other
    /// uses.
    pub align: u32,
}

impl FatHeader {
    pub fn read_from<R: Read>(read: &mut R) -> Self {
        let magic_n = read.read_u32::<NativeEndian>().unwrap();
        let magic = Magic::from_u32(magic_n);
        Self::read_after_magic(read, magic)
    }

    fn read_after_magic<R: Read>(read: &mut R, magic: Magic) -> Self {
        assert!(magic == Magic::FatMagic || magic == Magic::FatCigam);

        let n_fat_archs = read.read_u32::<BigEndian>().unwrap();

        let mut fat_archs = Vec::with_capacity(n_fat_archs as usize);

        for _ in 0..n_fat_archs {
            let cpu_type_n = read.read_i32::<BigEndian>().unwrap();
            let cpu_subtype_n = read.read_i32::<BigEndian>().unwrap();
            let cpu_type = CpuType::from_i32_i32(cpu_type_n, cpu_subtype_n);
            let offset = read.read_u32::<BigEndian>().unwrap();
            let size = read.read_u32::<BigEndian>().unwrap();
            let align = read.read_u32::<BigEndian>().unwrap();
            fat_archs.push(FatArch {
                cpu_type,
                offset,
                size,
                align,
            });
        }

        FatHeader { magic, fat_archs }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32::<BigEndian>(self.magic.to_u32()).unwrap();
        write
            .write_u32::<BigEndian>(self.fat_archs.len() as u32)
            .unwrap();

        for fat_arch in self.fat_archs.iter() {
            let (cpu_type_n, cpu_subtype_n) = fat_arch.cpu_type.to_i32_i32();
            write.write_i32::<BigEndian>(cpu_type_n).unwrap();
            write.write_i32::<BigEndian>(cpu_subtype_n).unwrap();
            write.write_u32::<BigEndian>(fat_arch.offset).unwrap();
            write.write_u32::<BigEndian>(fat_arch.size).unwrap();
            write.write_u32::<BigEndian>(fat_arch.align).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_header64() {
        let header = Header64 {
            magic: Magic::Magic64,
            cpu_type: CpuType::X86_64(CpuSubTypeX86_64::All),
            file_type: FileType::Object,
            n_cmds: 2,
            size_of_cmds: 42,
            flags: Flags::new(),
            reserved: 0,
        };

        let mut buf = Vec::new();

        header.write_into(&mut buf);

        assert_eq!(buf.len(), Header64::SIZE as usize);

        let read = Header64::read_from(&mut buf.as_slice());
        assert_eq!(read, header);
    }
}
