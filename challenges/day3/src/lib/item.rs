#[derive(Clone)]
pub struct Item {
    symbol: char,
    priority: u8,
}

const CAPITAL_OFFSET: u8 = 65 - 25;
const LOWERCASE_OFFSET: u8 = 97 - 96;

impl Item {    
    pub fn get_char(&self) -> char {
        self.symbol
    }
    pub fn get_priority(&self) -> u8 {
        self.priority
    }
}

impl From<&char> for Item {
    fn from(c: &char) -> Self {
        Self {
            symbol: *c,
            priority: match *c as u8 {
            bytecode if bytecode >= 65 && bytecode <= 90 => bytecode - CAPITAL_OFFSET,
            bytecode if bytecode >= 97 && bytecode <= 122  => bytecode - LOWERCASE_OFFSET,
            _ => panic!("The provided char {} was not valid!", c),
        }}
    }
}