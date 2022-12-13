use util::input_parser::parse_input_to_strings;
use std::io;

fn main() -> io::Result<()> {
    let input_strings = parse_input_to_strings(Some("EOF"));

    println!("Received {} lines", &input_strings.len());

    Ok(())
}
