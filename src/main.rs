// Code made by Antonio Noguerón
// on 03/27/2023

mod utils;

use crate::utils::file;
use crate::utils::lexer;
use crate::utils::lexer::Token;
use crate::utils::html_formatter;

fn main() {
    let file_path = String::from("src/test.txt");
    lexer_aritmetico(&file_path);
}


fn lexer_aritmetico(file_name: &String) {
    let cleared_lines = match file::get_cleared_lines_from_file(&file_name) {
        Some(cleared_lines) => cleared_lines,
        None => {
            println!("Error reading file");
            return;
        }
    };
    let tokenized_lines: Vec<Vec<Token>> = cleared_lines.iter().map(|line| lexer::tokenize_line(&line)).collect();
    html_formatter::get_html(&tokenized_lines);
    println!("Output file generated: output.html");
}