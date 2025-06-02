use std::env;

mod configuration;
mod elf_explorer;
mod elf_explorer;

// args[1] configuration file
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprint!("Missing configuration file name!\n");
        std::process::exit(1);
    }
    let config = configuration::Configuration::load_from_file(&args[1]);

    for file in &config.libraries {
        let elf_file = elf_explorer::ElfFile::new(file);
        let dependencies = elf_explorer::find_dependencies(&elf_file, &config.libraries_path);
    }
}
