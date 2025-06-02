use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::note::Note;
use elf::note::NoteGnuBuildId;
use elf::section::SectionHeader;

pub enum Type {
    Other,
    Executable,
    Archive,
    SharedLibrary
}

pub struct SoFileExplorer {

}



pub struct ElfFile {
    pub filename: String,
    pub elf_type: Type,
    pub depends_on: Vec<ElfFile>
}

impl ElfFile {
    pub fn new(filename: &str) -> ElfFile {
        let mut elf_file = ElfFile {
            filename: filename.to_string(),
            elf_type: Type::Other,
            depends_on: Vec::new()
        };
        let path = std::path::PathBuf::from(filename);
        let file_data = std::fs::read(path).expect("Could not read file.");
        let slice = file_data.as_slice();
        let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

        elf_file
    }
}

pub fn find_dependencies(elf_file: &ElfFile, library_paths: &String) -> Vec<ElfFile> {
    for item in &elf_file.depends_on {

    }
    Vec::new()
}
