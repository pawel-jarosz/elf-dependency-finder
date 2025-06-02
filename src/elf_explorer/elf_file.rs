use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::note::Note;
use elf::note::NoteGnuBuildId;
use elf::section::SectionHeader;

pub enum ElfType {
    Other,
    Executable,
    Archive,
    SharedLibrary
}

pub struct ElfFile {
    pub filename: String,
    pub elf_type: ElfType,
    pub depends_on: Vec<ElfFile>
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
        let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

        elf_file
    }
}

