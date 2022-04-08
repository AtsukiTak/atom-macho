use atom_macho::load_command::LoadCommand;
use std::borrow::Cow;

pub fn print_cmds(cmds: &[LoadCommand]) {
    println!("LoadCommand List");
    println!("--------------------");
    for (i, cmd) in cmds.iter().enumerate() {
        println!("{:<2} : {}", i, command_name(cmd));
    }
}

fn command_name(cmd: &LoadCommand) -> Cow<'static, str> {
    match cmd {
        LoadCommand::Segment64(seg, _) => format!("segment64 \"{}\"", seg.segname).into(),
        LoadCommand::Symtab(_) => "symtab".into(),
        LoadCommand::Dysymtab(_) => "dysymtab".into(),
        LoadCommand::Uuid(_) => "uuid".into(),
        LoadCommand::BuildVersion(_, _) => "buildversion".into(),
        LoadCommand::SourceVersion(_) => "sourceversion".into(),
        LoadCommand::Unsupported(cmd, _) => format!("unknown cmd [0x{:x}]", cmd).into(),
    }
}
