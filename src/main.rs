use std::env;

mod configuration;

// args[1] configuration file
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprint!("Missing configuration file name!\n");
        std::process::exit(1);
    }
    configuration::Configuration::load_from_file(&args[1]);

}
