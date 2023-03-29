use std::fs;

fn read_from_file(file_path: &String) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Some(contents),
        Err(_) => None,
    }
}

fn get_lines(text: &String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    for c in text.chars() {
        if c == '\n' {
            line.push(c);
            lines.push(line);
            line = String::new();
        } else {
            line.push(c);
        }
    }
    lines.push(line);
    lines
}

fn clear_lines_first_chars(lines: Vec<String>) -> Vec<String> {
    let mut cleared_lines: Vec<String> = Vec::new();
    for line in lines {
        let mut cleared_line = String::new();
        let mut found_first_char = false;
        for c in line.chars() {
            if found_first_char {
                cleared_line.push(c);
            } else {
                if c != ' ' && c != '\t' {
                    found_first_char = true;
                    cleared_line.push(c);
                }
            }
        }
        cleared_lines.push(cleared_line);
    }
    cleared_lines
}

pub fn get_cleared_lines_from_file(file_path: &String) -> Option<Vec<String>> {
    let contents = match read_from_file(file_path) {
        Some(contents) => contents,
        None => return None,
    };
    let lines = get_lines(&contents);
    let cleared_lines = clear_lines_first_chars(lines);
    Some(cleared_lines)
}