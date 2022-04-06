use crate::hex::Hex;
use atom_macho::{
    header::Header64, load_command::LoadCommand, nlist::NList64, reloc::RelocationInfo,
    string_table::StringTable,
};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct MachO {
    header: Header64,
    load_commands: Vec<LoadCommand>,
    sections: Vec<(Hex<Vec<u8>>, Vec<RelocationInfo>)>,
    symbol_tables: Vec<(Vec<NList64>, StringTable)>,
}

pub fn read_macho<T>(buf: &mut T) -> MachO
where
    T: Seek + Read,
{
    buf.seek(SeekFrom::Start(0)).unwrap();

    let header = Header64::read_from(buf);
    let endian = header.endian();

    let load_commands = (0..header.n_cmds)
        .map(|_| LoadCommand::read_from_in(buf, endian))
        .collect::<Vec<LoadCommand>>();

    // read sections
    let sections = load_commands
        .iter()
        .filter_map(|cmd| match cmd {
            LoadCommand::Segment64(_, sects) => Some(sects),
            _ => None,
        })
        .flatten()
        .map(|sect| {
            // section data
            buf.seek(SeekFrom::Start(sect.offset as u64)).unwrap();
            let mut data = vec![0; sect.size as usize];
            buf.read_exact(&mut data).unwrap();

            // reloc info
            buf.seek(SeekFrom::Start(sect.reloff as u64)).unwrap();
            let relocs = (0..sect.nreloc)
                .map(|_| RelocationInfo::read_from_in(buf, endian))
                .collect::<Vec<_>>();

            (Hex::new(data), relocs)
        })
        .collect::<Vec<_>>();

    // read symbol tables
    let symbol_tables = load_commands
        .iter()
        .filter_map(|cmd| match cmd {
            LoadCommand::Symtab(symtab) => Some(symtab),
            _ => None,
        })
        .map(|symtab| {
            // read nlists
            buf.seek(SeekFrom::Start(symtab.symoff as u64)).unwrap();
            let nlists = (0..symtab.nsyms)
                .map(|_| NList64::read_from_in(buf, endian))
                .collect::<Vec<NList64>>();

            // read string table
            buf.seek(SeekFrom::Start(symtab.stroff as u64)).unwrap();
            let mut string_table_data = vec![0; symtab.strsize as usize];
            buf.read_exact(&mut string_table_data).unwrap();
            let string_table = StringTable::from(string_table_data);

            (nlists, string_table)
        })
        .collect::<Vec<_>>();

    MachO {
        header,
        load_commands,
        sections,
        symbol_tables,
    }
}
