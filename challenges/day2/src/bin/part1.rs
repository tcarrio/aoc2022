use util::input_parser::parse_input_to_strings;
use day2::strategy::Strategy;

fn main() {
    let input = parse_input_to_strings();

    let total_score = input
        .iter()
        .filter(|s| Strategy::validate(s))
        // .count();
        .map(|line| Strategy::from_part1(line))
        .fold(0 as i64, |acc, strat| acc + strat.get_value());

    println!("Part 1: Total score is {}", total_score);
}
