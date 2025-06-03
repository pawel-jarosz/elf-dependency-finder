use goblin::elf::Elf;

#[derive(Clone)]
pub enum ElfType {
    Other,
    Executable,
    Archive,
    SharedLibrary
}

pub struct ElfFile {
    pub filename: String,
    pub elf_type: ElfType,
    pub depends_on: Vec<String>
}

impl ElfFile {
    pub fn new(filename: &str) -> Self {
        let mut elf_file = ElfFile {
            filename: filename.to_string(),
            elf_type: ElfType::Other,
            depends_on: Vec::new()
        };
        let path = std::path::PathBuf::from(filename);
        let file_data = std::fs::read(path).expect("Could not read file.");
        let slice = file_data.as_slice();
        let file = Elf::parse(&slice);

        if file.is_err() {
            return elf_file
        }

        let file = file.unwrap();
        elf_file.elf_type = match file.header.e_type {
            goblin::elf::header::ET_EXEC => ElfType::Executable,
            goblin::elf::header::ET_DYN => ElfType::SharedLibrary,
            goblin::elf::header::ET_REL => ElfType::Archive,
            _ => ElfType::Other
        };

        if let Some(dynamic) = file.dynamic {
            for dyn_entry in dynamic.dyns {
                if dyn_entry.d_tag == goblin::elf::dynamic::DT_NEEDED {
                    if let Some(name) = file.dynstrtab.get(dyn_entry.d_val as usize) {
                        elf_file.depends_on.push(name.unwrap().to_string());
                    }
                }
                if dyn_entry.d_tag == goblin::elf::dynamic::DT_SONAME {
                    if let Some(name) = file.dynstrtab.get(dyn_entry.d_val as usize) {
                        elf_file.filename = name.unwrap().to_string();
                    }
                }
            }
        }
        elf_file
    }
}

