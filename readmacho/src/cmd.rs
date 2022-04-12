use atom_macho::load_command::{
    build_version::{BuildToolVersion, BuildVersionCommand, Platform, Tool},
    segment64::{Section64, SectionAttr, SectionType, SegmentCommand64},
    unix_thread::{Flavor, ThreadState},
    DysymtabCommand, LoadCommand, SourceVersionCommand, SymtabCommand, UnixThreadCommand,
    UuidCommand,
};

pub fn print_cmd(cmds: &[LoadCommand], idx: usize) {
    let cmd = match cmds.get(idx) {
        None => {
            panic!("Load Command {} does not exist", idx);
        }
        Some(cmd) => cmd,
    };

    println!("Load Command {}", idx);
    println!("------------------------");

    match cmd {
        LoadCommand::Segment64(seg, sects) => {
            print_segment64(seg);
            let n_prev_sects = count_prev_sects(cmds, idx);
            for (i, sect) in sects.iter().enumerate() {
                println!("-- section");
                println!("{:<10} : {}", "index", i + n_prev_sects + 1);
                print_section(sect);
            }
        }
        LoadCommand::Symtab(symtab) => {
            print_symtab(symtab);
        }
        LoadCommand::UnixThread(thread) => {
            print_unixthread(thread);
        }
        LoadCommand::Dysymtab(dysymtab) => {
            print_dysymtab(dysymtab);
        }
        LoadCommand::Uuid(uuid) => {
            print_uuid(uuid);
        }
        LoadCommand::BuildVersion(build_ver, tool_vers) => {
            print_buildversion(build_ver);
            for tool_ver in tool_vers {
                print_tool_version(tool_ver);
            }
        }
        LoadCommand::SourceVersion(source_ver) => {
            print_source_version(source_ver);
        }
        LoadCommand::Unsupported(_, _) => {}
    }
}

fn print_segment64(seg: &SegmentCommand64) {
    println!("{:<10} : {}", "cmd", "LC_SECGMENT64");
    println!("{:<10} : {}", "cmdsize", seg.cmdsize);
    println!("{:<10} : \"{}\"", "segname", seg.segname.as_str());
    println!("{:<10} : 0x{:x}", "vmaddr", seg.vmaddr);
    println!("{:<10} : 0x{:x}", "vmsize", seg.vmsize);
    println!("{:<10} : {}", "fileoff", seg.fileoff);
    println!("{:<10} : {}", "filesize", seg.filesize);
    println!("{:<10} : 0b{:03b}", "maxprot", seg.maxprot);
    println!("{:<10} : 0b{:03b}", "initprot", seg.initprot);
    println!("{:<10} : {}", "nsects", seg.nsects);
    println!("{:<10} : 0x{:x}", "flags", seg.flags);
}

fn print_section(sect: &Section64) {
    println!("{:<10} : \"{}\"", "sectname", sect.sectname.as_str());
    println!("{:<10} : 0x{:x}", "addr", sect.addr);
    println!("{:<10} : 0x{:x}", "size", sect.size);
    println!("{:<10} : {}", "fileoff", sect.offset);
    println!("{:<10} : 2^{}", "align", sect.align);
    println!("{:<10} : {}", "reloff", sect.reloff);
    println!("{:<10} : {}", "nreloc", sect.nreloc);

    let sect_ty = match sect.flags.1 {
        SectionType::Regular => "regular",
        SectionType::Zerofill => "zerofill",
        SectionType::CstringLiterals => "cstring literals",
        SectionType::FourByteLiterals => "4 byte literals",
        SectionType::EightByteLiterals => "8 byte literals",
        SectionType::LiteralPointers => "literals pointers",
        SectionType::Coalesced => "coalesced",
    };
    println!("{:<10} : {}", "type", sect_ty);

    for (i, attr) in sect.flags.0.iter().enumerate() {
        let attr_str = match attr {
            SectionAttr::PureInstructions => "pure instructions",
            SectionAttr::NoToc => "no toc",
            SectionAttr::StripStaticSyms => "strip static syms",
            SectionAttr::LiveSupport => "live support",
            SectionAttr::Debug => "debug",
            SectionAttr::SomeInstructions => "some instructions",
            SectionAttr::ExtReloc => "ext reloc",
            SectionAttr::LocReloc => "loc reloc",
        };
        let key_str = if i == 0 { "attr" } else { "" };
        println!("{:<10} : {}", key_str, attr_str);
    }
}

// returns a number of sections placed before segment `seg_idx`.
fn count_prev_sects(cmds: &[LoadCommand], seg_idx: usize) -> usize {
    let mut n = 0;

    for i in 0..seg_idx {
        if let LoadCommand::Segment64(_, sects) = &cmds[i] {
            n += sects.len();
        }
    }

    n
}

fn print_symtab(cmd: &SymtabCommand) {
    println!("{:<10} : {}", "cmd", "LC_SYMTAB");
    println!("{:<10} : {}", "cmdsize", cmd.cmdsize);
    println!("{:<10} : {}", "symoff", cmd.symoff);
    println!("{:<10} : {}", "nsyms", cmd.nsyms);
    println!("{:<10} : {}", "stroff", cmd.stroff);
    println!("{:<10} : {}", "strsize", cmd.strsize);
}

fn print_unixthread(cmd: &UnixThreadCommand) {
    println!("{:<10} : {}", "cmd", "LC_UNIXTHREAD");
    println!("{:<10} : {}", "cmdsize", cmd.cmdsize);

    match cmd.flavor {
        Flavor::ThreadStateX86_64 => println!("{:<10} : x86_THREAD_STATE64", "flavor"),
        Flavor::Unknown(n) => println!("{:<10} : unknown {}", "flavor", n),
    };

    println!("{:<10} : {}", "count", cmd.count);

    match &cmd.state {
        ThreadState::X86_64(state) => {
            println!("{:<10} : {:<7} 0x{:016x}", "state", "rax", state.__rax);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rbx", state.__rbx);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rcx", state.__rcx);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rdx", state.__rdx);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rdi", state.__rdi);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rsi", state.__rsi);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rbp", state.__rbp);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rsp", state.__rsp);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r8", state.__r8);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r9", state.__r9);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r10", state.__r10);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r11", state.__r11);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r12", state.__r12);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r13", state.__r13);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r14", state.__r14);
            println!("{:<10} : {:<7} 0x{:016x}", "", "r15", state.__r15);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rip", state.__rip);
            println!("{:<10} : {:<7} 0x{:016x}", "", "rflags", state.__rflags);
            println!("{:<10} : {:<7} 0x{:016x}", "", "cs", state.__cs);
            println!("{:<10} : {:<7} 0x{:016x}", "", "fs", state.__fs);
            println!("{:<10} : {:<7} 0x{:016x}", "", "gs", state.__gs);
        }
        ThreadState::Unknown(_) => {}
    }
}

fn print_dysymtab(cmd: &DysymtabCommand) {
    println!("{:<10} : {}", "cmd", "LC_DYSYMTAB");
    println!("{:<10} : {}", "cmdsize", cmd.cmdsize);
    println!("{:<10} : {}", "ilocalsym", cmd.ilocalsym);
    println!("{:<10} : {}", "nlocalsym", cmd.nlocalsym);
    println!("{:<10} : {}", "iextdefsym", cmd.iextdefsym);
    println!("{:<10} : {}", "nextdefsym", cmd.nextdefsym);
    println!("{:<10} : {}", "iundefsym", cmd.iundefsym);
    println!("{:<10} : {}", "nundefsym", cmd.nundefsym);
    println!("{:<10} : {}", "tocoff", cmd.tocoff);
    println!("{:<10} : {}", "ntoc", cmd.ntoc);
    println!("{:<10} : {}", "modtaboff", cmd.modtaboff);
    println!("{:<10} : {}", "nmodtab", cmd.nmodtab);
    println!("{:<10} : {}", "extrefsymoff", cmd.extrefsymoff);
    println!("{:<10} : {}", "nextrefsyms", cmd.nextrefsyms);
    println!("{:<10} : {}", "indirectsymoff", cmd.indirectsymoff);
    println!("{:<10} : {}", "nindirectsyms", cmd.nindirectsyms);
    println!("{:<10} : {}", "extreloff", cmd.extreloff);
    println!("{:<10} : {}", "nextrel", cmd.nextrel);
    println!("{:<10} : {}", "locreloff", cmd.locreloff);
    println!("{:<10} : {}", "nlocrel", cmd.nlocrel);
}

fn print_buildversion(cmd: &BuildVersionCommand) {
    println!("{:<10} : {}", "cmd", "LC_BUILD_VERSION");
    println!("{:<10} : {}", "cmdsize", cmd.cmdsize);

    let platform = match cmd.platform {
        Platform::MacOS => "macOS",
        Platform::IOS => "iOS",
        Platform::TvOS => "tvOS",
        _ => "other",
    };
    println!("{:<10} : {}", "platform", platform);

    println!(
        "{:<10} : {}.{}.{}",
        "minos", cmd.minos.major, cmd.minos.minor, cmd.minos.release
    );

    println!(
        "{:<10} : {}.{}.{}",
        "sdk", cmd.sdk.major, cmd.sdk.minor, cmd.sdk.release
    );
}

fn print_tool_version(ver: &BuildToolVersion) {
    println!("--- tool version");

    let tool_str = match ver.tool {
        Tool::Clang => "clang",
        Tool::Swift => "swift",
        Tool::LD => "ld",
    };
    println!("{:<10} : {}", "tool", tool_str);

    println!("{:<10} : {}", "version", ver.version);
}

fn print_source_version(cmd: &SourceVersionCommand) {
    println!("{:<10} : {}", "cmd", "LC_SOURCE_VERSION");
    println!("{:<10} : {}", "cmdsize", cmd.cmdsize);

    let ver = cmd.version;
    println!(
        "{:<10} : {}.{}.{}.{}.{}",
        "version",
        ver.a(),
        ver.b(),
        ver.c(),
        ver.d(),
        ver.e()
    );
}

fn print_uuid(cmd: &UuidCommand) {
    println!("{:<10} : {}", "cmd", "LC_UUID");
    println!("{:<10} : {}", "cmdsize", cmd.cmdsize);

    print!("{:<10} : 0x", "uuid");
    for b in cmd.uuid {
        print!("{:02x}", b);
    }
    println!("");
}
