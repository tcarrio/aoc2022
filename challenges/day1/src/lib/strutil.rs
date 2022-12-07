pub fn is_numeric_string(input_str: &str) -> bool {
  input_str
      .chars()
      .filter(|c| c != &'\n' && c != &'\r')
      .fold(true, |a, c| a && c.is_numeric())
}
