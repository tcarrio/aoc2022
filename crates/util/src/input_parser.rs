use std::io;

pub fn parse_input_to_strings() -> Vec<String> {
    let mut string_lines: Vec<String> = vec![];
    
    let lines = io::stdin().lines();
    for line in lines {
        if line.is_ok() {
            string_lines.push(line.unwrap());
        }
    }

    string_lines
}