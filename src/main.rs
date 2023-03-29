mod utils;

use regex::Regex;
use crate::utils::file;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));

    let file_path = String::from("src/test.txt");
    let contents = match file::read_from_file(&file_path) {
        Some(contents) => contents,
        None => String::from(""),
    };

    let lines = file::get_lines(&contents);
    let cleared_lines = file::clear_lines_first_chars(lines);

    for line in cleared_lines {
        print!("{}", line);
    }
}

pub enum TokenKind {
    Integer,
    Float,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Power,
    LParen,
    RParen,
    Identifier,
    Comment,
    Spaces,
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}