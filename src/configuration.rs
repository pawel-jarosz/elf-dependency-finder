use std::fs;

use yaml_rust2::YamlLoader;

pub struct Configuration {
    libraries: Vec<String>,
    excluded_libraries: Vec<String>,
    libraries_path: String
}

impl Configuration {
    pub fn load_from_str(config: &str) -> Configuration {
        let mut configuration = Configuration {
            libraries: Vec::new(),
            excluded_libraries: Vec::new(),
            libraries_path: String::new()
        };

        let docs = YamlLoader::load_from_str(config).unwrap();
        let doc = &docs[0];

        for item in doc["libraries"].as_vec().expect("Invalid files section") {
            configuration.libraries.push(item.as_str().unwrap().to_string());
        }

        for item in doc["excluded_libraries"].as_vec().expect("Invalid excluded files section") {
            configuration.excluded_libraries.push(item.as_str().unwrap().to_string());
        }
        configuration.libraries_path = doc["libraries_path"].as_str().unwrap().to_string();

        configuration
    }

    pub fn load_from_file(filename: &str) -> Configuration {
        let content = fs::read_to_string(filename)
            .expect("Should have been able to read the file");
        Configuration::load_from_str(&content)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const content: &str = "
    libraries:
        - libtest.so
    excluded_libraries:
        - libm.so
        - libglibc.so
    libraries_path: /usr/lib
    ";

    #[test]
    fn test_parsing_from_string() {
        let configuration = Configuration::load_from_str(&content);
        assert_eq!(configuration.libraries.join(", "), "libtest.so");
        assert_eq!(configuration.excluded_libraries.join(", "), "libm.so, libglibc.so");
        assert_eq!(configuration.libraries_path, "/usr/lib");
    }
}