use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Symbol {
    One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Stop,
        Plus2,
        Plus4,
        ChangeColor,
        ChangeDirection,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Symbol::*;
        let symbol = match self {
            One => "1",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Stop => "Ø",
            Plus2 => "+2",
            Plus4 => "+4",
            ChangeColor => "CC",
            ChangeDirection => "CD",
        };

        write!(f, "{}", symbol)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum Color {
    RED = 31,
    YELLOW = 33,
    BLUE = 34,
    GREEN = 32,
    SPECIAL = 0,
}

#[derive(Debug, Clone)]
pub struct Card {
symbol: Symbol,
            color: Color,
}

impl Card {

    pub fn new(color: Color, symbol: Symbol) -> Card {
        Card { color, symbol }
    }

    pub fn effect(&self) {
        match &self.symbol {
            Symbol::Stop => 0,   // STOP: next player can't play,
                Symbol::Plus2 => 0 , // PLUS_2: NEXT PLAYER draw 2 cards
                Symbol::Plus4 => 0, // PLUS_4: NEXT PLAYER draw 4 cards and current player chose a new color
                Symbol::ChangeColor => 0,   // CHANGE_COLOR: Current player chose a new color
                Symbol::ChangeDirection => 0,   // CHANGE_DIRECTION: Inverse the play order
                _ => 0, // ONE-NINE: do nothing

        };
        return;
    }

    pub fn same_symbol_as(&mut self, card: &Card) -> bool {
        return self.symbol == (*card).symbol;
    }

    pub fn same_color_as(&mut self, card: &Card) -> bool {
       return self.color == (*card).color;
    }
}

// Display a card
// the font color is the same as the card color
// then reset the font color for the following text
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{001b}[{}m{}\u{001b}[0m", self.color as u32, self.symbol)
    }

}
