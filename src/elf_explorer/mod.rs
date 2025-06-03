mod elf_file;

use std::collections::HashMap;
use goblin::elf::Elf;
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
            let filename = entry.file_name().to_str().unwrap();
            let path_to_file = entry.path().to_str().unwrap();

            let has_so_extension = filename.contains(".so");
            if has_so_extension {
                let elf_file = ElfFile::new(path_to_file);

                if elf_file.filename != filename {
                    println!("{} != {}", elf_file.filename, filename);
                }
                
                let is_available = self.available_libraries.contains_key(&elf_file.filename);
                if  is_available {
                    self.available_libraries.get_mut(filename).unwrap().1.push(path_to_file.to_string());
                }
            }
        }
        found_libraries
    }
}


pub fn find_dependencies(elf_file: &ElfFile, library_paths: &String) -> Vec<ElfFile> {
    for item in &elf_file.depends_on {

    }
    Vec::new()
}
