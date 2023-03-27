use std::fs;

fn main() {
    let file_path = String::from("src/test.txt");
    println!("Reading from file: {}", &file_path);
    let contents = match read_from_file(&file_path) {
        Some(contents) => contents,
        None => String::from(""),
    };
    println!("Contents: {}", contents);
}

fn read_from_file(file_path: &String) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Some(contents),
        Err(_) => None,
    }
}