use util::input_parser::parse_input_to_strings;
use day1::tracker::ElfTracker;
use std::io;

fn main() -> io::Result<()> {
    let tracker = ElfTracker::from(parse_input_to_strings());
    
    println!("The total number of elves found was {}", tracker.len());
    println!("The number of calories held by the elf with the most calories is {}", tracker.get_elf_with_most_calories().total);

    Ok(())
}
