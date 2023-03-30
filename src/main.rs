// Code made by Antonio Noguerón
// on 03/27/2023

mod utils;

use crate::utils::file;
use crate::utils::lexer;
use crate::utils::lexer::Token;
use crate::utils::html_formatter;

fn main() {
    let file_path = String::from("src/test.txt");
    let cleared_lines = match file::get_cleared_lines_from_file(&file_path) {
        Some(cleared_lines) => cleared_lines,
        None => return,
    };
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
    for line in cleared_lines {
        tokenized_lines.push(lexer::tokenize_line(&line));
    }
    html_formatter::get_html(&tokenized_lines);
}
