use std::fs;

use crate::errors::GeneratorError;

pub fn create_file(path: &str, content: &str) -> Result<(), GeneratorError> {
    fs::write(path, content).map_err(GeneratorError::FileCreationError)
}

pub fn create_directory(path: &str) -> Result<(), GeneratorError> {
    fs::create_dir_all(path).map_err(GeneratorError::DirectoryCreationError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_create_file() {
        let path = "test_file.txt";
        let content = "Hello, world!";
        create_file(path, content).expect("Unable to create file");

        let file_content = fs::read_to_string(path).expect("Unable to read file");
        assert_eq!(file_content, content);

        fs::remove_file(path).expect("Unable to delete test file");
    }

    #[test]
    fn test_create_directory() {
        let path = "test_dir";
        create_directory(path).expect("Unable to create directory");

        assert!(Path::new(path).exists() && Path::new(path).is_dir());

        fs::remove_dir(path).expect("Unable to delete test directory");
    }
}
