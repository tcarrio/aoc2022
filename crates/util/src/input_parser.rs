use std::fs;
use std::io;
use std::io::Lines;
use std::io::StdinLock;

pub fn parse_input_to_strings(eof: Option<&str>) -> Vec<String> {
    let lines = io::stdin().lines();
    
    match eof {
        Some(eof) => parse_input_to_strings_with_eof(lines, eof),
        None => parse_input_to_strings_without_eof(lines),
    }
}

fn parse_input_to_strings_with_eof(lines: Lines<StdinLock>, eof: &str) -> Vec<String> {
    let mut string_lines: Vec<String> = vec![];
    let str_eof = eof.to_string();

    for line in lines {
        match line {
            Err(_) => continue,
            Ok(val) if val == str_eof => break,
            Ok(val) => string_lines.push(val),
        };
    }

    string_lines
}

fn parse_input_to_strings_without_eof(lines: Lines<StdinLock>) -> Vec<String> {
    let mut string_lines: Vec<String> = vec![];

    for line in lines {
        match line {
            Err(_) => continue,
            Ok(val) => string_lines.push(val),
        };
    }

    string_lines
}

pub fn parse_file_to_strings(filepath: &str) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(filepath);

    if let Err(message) = content  {
        return Err(message.to_string());
    }

    Ok(content.unwrap().lines().map(|line| line.to_string()).collect())
}