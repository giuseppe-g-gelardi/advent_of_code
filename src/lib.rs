use std::fs;

pub fn read_input(file_path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}
