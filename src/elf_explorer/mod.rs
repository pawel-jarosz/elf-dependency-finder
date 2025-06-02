

mod elf_file;

use std::collections::HashMap;
use walkdir::WalkDir;

pub use elf_file::{ElfFile, ElfType};

pub struct SoFileExplorer {
    available_libraries: HashMap<String, (ElfFile, Vec<String>)>
}

impl SoFileExplorer {
    pub fn new() -> Self {
        SoFileExplorer {
            available_libraries: HashMap::new()
        }
    }

    pub fn add_from_location(&mut self, path: &str) -> usize {
        let found_libraries = 0_usize;
        for entry in WalkDir::new(path).follow_links(true) {
            let entry = entry.unwrap();
            if entry.path().file_name().unwrap().to_str().unwrap().ends_with(".so") {
                
            }

        }
        4
    }
}



pub fn find_dependencies(elf_file: &ElfFile, library_paths: &String) -> Vec<ElfFile> {
    for item in &elf_file.depends_on {

    }
    Vec::new()
}
