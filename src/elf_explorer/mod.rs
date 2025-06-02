

mod elf_file;

pub use elf_file::{ ElfFile, ElfType};

pub struct SoFileExplorer {

}

impl SoFileExplorer {
    pub fn new() -> Self {
        SoFileExplorer {}
    }
}



pub fn find_dependencies(elf_file: &ElfFile, library_paths: &String) -> Vec<ElfFile> {
    for item in &elf_file.depends_on {

    }
    Vec::new()
}
