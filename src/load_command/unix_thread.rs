use crate::io::{Endian, ReadExt as _, WriteExt as _};
use std::io::{Read, Write};

/*
 * Thread commands contain machine-specific data structures suitable for
 * use in the thread state primitives.  The machine specific data structures
 * follow the struct thread_command as follows.
 * Each flavor of machine specific data structure is preceded by an uint32_t
 * constant for the flavor of that data structure, an uint32_t that is the
 * count of uint32_t's of the size of the state data structure and then
 * the state data structure follows.  This triple may be repeated for many
 * flavors.  The constants for the flavors, counts and state data structure
 * definitions are expected to be in the header file <machine/thread_status.h>.
 * These machine specific data structures sizes must be multiples of
 * 4 bytes.  The cmdsize reflects the total size of the thread_command
 * and all of the sizes of the constants for the flavors, counts and state
 * data structures.
 *
 * For executable objects that are unix processes there will be one
 * thread_command (cmd == LC_UNIXTHREAD) created for it by the link-editor.
 * This is the same as a LC_THREAD, except that a stack is automatically
 * created (based on the shell's limit for the stack size).  Command arguments
 * and environment variables are copied onto that stack.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnixThreadCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub flavor: Flavor,
    /// size of the thread state data, in number of 32-bit integers. The thread state data
    /// structure must be fully padded to 32-bit alignment.
    pub count: u32,
    pub state: ThreadState,
}

impl UnixThreadCommand {
    pub const TYPE: u32 = 0x5;

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let cmd = read.read_u32_in(endian);
        let cmdsize = read.read_u32_in(endian);
        let flavor = Flavor::read_from_in(read, endian);
        let count = read.read_u32_in(endian);

        let state = match flavor {
            Flavor::ThreadStateX86_64 => {
                let state = StateX86_64::read_from_in(read, endian);
                ThreadState::X86_64(state)
            }
            Flavor::Unknown(_) => {
                let mut state = Vec::with_capacity(count as usize * 4);
                state.resize(count as usize * 4, 0);
                read.read_exact(&mut state).unwrap();
                ThreadState::Unknown(state)
            }
        };

        UnixThreadCommand {
            cmd,
            cmdsize,
            flavor,
            count,
            state,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.cmd);
        write.write_u32_native(self.cmdsize);
        self.flavor.write_into(write);
        write.write_u32_native(self.count);

        match &self.state {
            ThreadState::X86_64(state) => state.write_into(write),
            ThreadState::Unknown(state) => write.write_all(&state).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flavor {
    ThreadStateX86_64,
    Unknown(u32),
}

impl Flavor {
    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        match read.read_u32_in(endian) {
            4 => Flavor::ThreadStateX86_64,
            n => Flavor::Unknown(n),
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        match self {
            Flavor::ThreadStateX86_64 => write.write_u32_native(4),
            Flavor::Unknown(n) => write.write_u32_native(*n),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadState {
    X86_64(StateX86_64),
    Unknown(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateX86_64 {
    pub __rax: u64,
    pub __rbx: u64,
    pub __rcx: u64,
    pub __rdx: u64,
    pub __rdi: u64,
    pub __rsi: u64,
    pub __rbp: u64,
    pub __rsp: u64,
    pub __r8: u64,
    pub __r9: u64,
    pub __r10: u64,
    pub __r11: u64,
    pub __r12: u64,
    pub __r13: u64,
    pub __r14: u64,
    pub __r15: u64,
    pub __rip: u64,
    pub __rflags: u64,
    pub __cs: u64,
    pub __fs: u64,
    pub __gs: u64,
}

impl StateX86_64 {
    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        StateX86_64 {
            __rax: read.read_u64_in(endian),
            __rbx: read.read_u64_in(endian),
            __rcx: read.read_u64_in(endian),
            __rdx: read.read_u64_in(endian),
            __rdi: read.read_u64_in(endian),
            __rsi: read.read_u64_in(endian),
            __rbp: read.read_u64_in(endian),
            __rsp: read.read_u64_in(endian),
            __r8: read.read_u64_in(endian),
            __r9: read.read_u64_in(endian),
            __r10: read.read_u64_in(endian),
            __r11: read.read_u64_in(endian),
            __r12: read.read_u64_in(endian),
            __r13: read.read_u64_in(endian),
            __r14: read.read_u64_in(endian),
            __r15: read.read_u64_in(endian),
            __rip: read.read_u64_in(endian),
            __rflags: read.read_u64_in(endian),
            __cs: read.read_u64_in(endian),
            __fs: read.read_u64_in(endian),
            __gs: read.read_u64_in(endian),
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u64_native(self.__rax);
        write.write_u64_native(self.__rbx);
        write.write_u64_native(self.__rcx);
        write.write_u64_native(self.__rdx);
        write.write_u64_native(self.__rdi);
        write.write_u64_native(self.__rsi);
        write.write_u64_native(self.__rbp);
        write.write_u64_native(self.__rsp);
        write.write_u64_native(self.__r8);
        write.write_u64_native(self.__r9);
        write.write_u64_native(self.__r10);
        write.write_u64_native(self.__r11);
        write.write_u64_native(self.__r12);
        write.write_u64_native(self.__r13);
        write.write_u64_native(self.__r14);
        write.write_u64_native(self.__r15);
        write.write_u64_native(self.__rip);
        write.write_u64_native(self.__rflags);
        write.write_u64_native(self.__cs);
        write.write_u64_native(self.__fs);
        write.write_u64_native(self.__gs);
    }
}
