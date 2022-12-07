use std::io::Result;

use crate::input_parser;

const EOF_STR: &str = "EOF";

#[derive(Clone)]
pub struct Cli<F: Clone + FnOnce(Vec<String>) -> Result<()>> {
    name: String,
    filepath: Option<String>,
    func: F,
}

impl<F> Cli<F>
where
    F: Clone + FnOnce(Vec<String>) -> Result<()>,
{
    pub fn run(&self) -> Result<()> {
        println!("Executing function '{}'", self.name);

        let lines = match &self.filepath {
            Some(filepath) => input_parser::parse_file_to_strings(&filepath[..]),
            None => Ok(input_parser::parse_input_to_strings(Some(EOF_STR))),
        };

        if let Err(msg) = lines {
            println!("An unexpected error occurred reading input");
            println!("{}", msg);

            return Ok(());
        }

        let func = self.func.clone();

        let val = func(lines.unwrap());

        val
    }
}

#[cfg(test)]
mod tests {
    use crate::eol::get_eol;

    use super::*;

    // Import the necessary modules
    use std::fs::OpenOptions;
    use std::io::Write;

    fn seed_file(filepath: &str, content: Vec<&str>) {
        let eol_bytes = get_eol().as_bytes();

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filepath)
            .expect(&format!("Failed to open {}", filepath));

        for line in content {
            let full_line = [line.as_bytes(), eol_bytes].concat();

            file.write_all(&full_line)
                .expect(&format!("Could not write to {}", filepath));
        }
    }

    #[test]
    fn create_cli_with_stdin() {
        let cli = Cli {
            filepath: None,
            func: |lines: Vec<String>| Ok(()),
            name: String::from("create_cli_with_stdin"),
        };

        let outcome = cli.run();

        assert_eq!(true, outcome.is_ok());
    }

    #[test]
    fn create_cli_with_filename() {
        let cli = Cli {
            filepath: None,
            func: |lines: Vec<String>| Ok(()),
            name: String::from("create_cli_with_filename"),
        };

        let outcome = cli.run();

        assert_eq!(true, outcome.is_ok());
    }

    #[test]
    fn create_cli_with_filename_and_eof() {
        assert_eq!(true, false);
    }
}
