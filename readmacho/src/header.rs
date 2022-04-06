use atom_macho::header::*;

pub fn print_header(header: Header64) {
    print("cpu type", format_cpu_type(header.cpu_type));
    print("file type", format_file_type(header.file_type));
    print("num commands", header.n_cmds);
    print("size of commands", header.size_of_cmds);
    if !header.flags.is_empty() {
        let mut flags = header.flags.iter();

        let flag1 = flags.next().unwrap();
        print("flags", format_flag(flag1));

        for flag in flags {
            print("", format_flag(flag));
        }
    }
}

fn print(key: &str, val: impl std::fmt::Display) {
    println!("{:<20} |  {:<20}", key, val);
}

fn format_cpu_type(cpu: CpuType) -> &'static str {
    match cpu {
        CpuType::X86(CpuSubTypeX86::All) => "x86",
        CpuType::X86_64(CpuSubTypeX86_64::All) => "x86_64",
    }
}

fn format_file_type(file: FileType) -> &'static str {
    use FileType::*;

    match file {
        Object => "object",
        Execute => "executable",
        FVMLib => "fvmlib",
        Core => "core",
        Preload => "preload",
        Dylib => "dylib",
        Dylinker => "dylinker",
        Bundle => "bundle",
        Dsym => "dsym",
    }
}

fn format_flag(flag: Flag) -> &'static str {
    use Flag::*;

    match flag {
        NoUndefs => "no_undefs",
        IncrLink => "incr_link",
        DyldLink => "dyld_link",
        BindAtLoad => "bind_at_load",
        PreBound => "pre_bound",
        SplitSegs => "split_segs",
        TwoLevel => "two_level",
        ForceFlat => "force_flat",
        NoMultiDefs => "no_multi_defs",
        NoFixPreBinding => "no_fix_pre_binding",
        PreBindable => "pre_bindable",
        AllModsBound => "all_mods_bound",
        SubsectionsViaSymbols => "subsections_via_symbols",
        Canonical => "canonical",
        Pie => "pie",
        HasTlvDescriptors => "has_tlv_descriptors",
    }
}
