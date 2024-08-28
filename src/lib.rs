mod process_ini_file;

use process_ini_file::*;

#[allow(dead_code)]
pub fn reset_ini_file(filename: &String, keys_to_reset: &Vec<String>) -> Result<(), IniError> {
    if filename.len() == 0 {
        return Err(IniError::NoFilenameGiven);
    }

    // Read INI file into memory
    let mut file_lines = match read_lines_from_file(&filename) {
        Ok(lines_read) => lines_read,
        Err(_e) => {
            // err_msgbox(&e, None);
            return Err(IniError::CannotReadFile);
        }
    };

    // If INI file is empty then no need to process it. Just silently exit.
    if file_lines.len() == 0 {
        return Err(IniError::EmptyFile);
    }

    // Reset (delete) values of given keys
    reset_keys_values(&mut file_lines, &keys_to_reset);

    // Write modified lines to source INI file
    match write_lines_to_file(&filename, &file_lines) {
        Ok(_) => (),
        Err(_e) => {
            // If there was any error during writing to output file then
            // display it to the user.
            // err_msgbox(&e, None);
            return Err(IniError::CannotWriteFile);
        }
    };

    Ok(())
}

#[allow(dead_code, unused)]
fn reset_ini_buffer(buffer: &String, keys_to_reset: &Vec<String>) -> Result<(), IniError> {
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn empty_filename_reset_ini_file() {
        let filename = String::from("");
        let keys = vec![String::from("")];
        let result = reset_ini_file(&filename, &keys);
        assert_eq!(result, Err(IniError::NoFilenameGiven));
    }
}
