#[derive(PartialEq, Debug)]
pub enum IniError {
    FileNotFound,
    NoFilenameGiven,
    NoKeyNameGiven,
    CannotWriteFile,
    CannotReadFile,
    EmptyFile,
    EmptyBuffer,
    ParsingError,
}

pub fn read_lines_from_file(filename: &String) -> Result<Vec<String>, String> {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    if filename == "" {
        return Err(String::from("No filename is given"));
    }

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("Error opening file: {}\n\nError:{}", filename, e)),
    };

    // Read all lines (from INI file) into vector of strings
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|l| l.unwrap_or("Could not parse line".to_string()))
        .collect();

    Ok(lines)
}

pub fn reset_keys_values(lines: &mut Vec<String>, keys: &Vec<String>) {
    for line in lines {
        // Trim all (leading and trailing) white spaces from line before processing it
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed == "" {
            continue;
        }
        // Skip lines we don't want to process
        match trimmed.chars().nth(0).unwrap_or('\0') {
            // Skip comments and section declaration
            '#' | ';' | '[' => continue,
            // Let rest of cases get check by (any) following guard clauses
            _ => (),
        }

        // Confirm there is a key value pair in this line, otherwise skip the line
        if trimmed.contains('=') {
            // Does the key's name matches with one of the given keys' names
            if key_matches(trimmed, &keys) {
                // If key name matches then reset it's value
                reset_key(line);
            }
        }
    }
}

fn key_matches(line: &str, keys: &Vec<String>) -> bool {
    let (line_key, _) = match line.split_once("=") {
        None => (line, ""),
        Some(s) => s,
    };

    for key in keys {
        // Ignore text case
        if line_key.to_lowercase() == key.to_lowercase() {
            return true;
        }
    }

    return false;
}

fn reset_key(line: &mut String) {
    let (key, _) = match line.split_once("=") {
        None => (line.as_str(), ""),
        Some(s) => s,
    };

    *line = key.to_string() + "=";
}

pub fn write_lines_to_file(filename: &String, lines: &Vec<String>) -> Result<String, String> {
    use std::fs;

    // Flatten Vec<String> into a str slice and write it to the output file.
    // Handle error and return a formatted error message string.
    return match fs::write(filename, lines.join("\n")) {
        Err(e) => Err(format!(
            "Error writing to file: {}\n\nError: {}",
            filename, e
        )),
        Ok(_) => Ok(String::new()),
    };
}
