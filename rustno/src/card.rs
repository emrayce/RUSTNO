#[derive(Debug, Clone)]
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Color {
    RED,
    YELLOW,
    BLUE,
    GREEN,
    BLACK,
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
}
