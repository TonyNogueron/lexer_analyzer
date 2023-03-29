mod utils;

use regex::Regex;
use crate::utils::file;
use crate::utils::lexer;
use crate::utils::lexer::Token;


fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));

    let file_path = String::from("src/test.txt");
    let cleared_lines = match file::get_cleared_lines_from_file(&file_path) {
        Some(cleared_lines) => cleared_lines,
        None => return,
    };
    for line in cleared_lines {
        let tokens: Vec<Token> = lexer::tokenize_line(&line);
        println!("Line: {:?}", line);
        for token in tokens {
            println!("{:?} ", token);
        }
        println!("----------------");
    }
}