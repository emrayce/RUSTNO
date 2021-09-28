#[derive(Debug)]
pub enum Symbol {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    STOP,
    PLUS_2,
    PLUS_4,
    CHANGE_COLOR,
    CANGE_DIRECTION,
}

#[derive(Debug)]
pub enum Color {
    RED,
    YELLOW,
    BLUE,
    GREEN,
    BLACK,
}

#[derive(Debug)]
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
            STOP => 0,   // STOP: next player can't play,
            PLUS_2 => 0 , // PLUS_2: NEXT PLAYER draw 2 cards
            PLUS_4 => 0, // PLUS_4: NEXT PLAYER draw 4 cards and current player chose a new color
            CHANGE_COLOR => 0,   // CHANGE_COLOR: Current player chose a new color
            CHANGE_DIRECTION => 0,   // CHANGE_DIRECTION: Inverse the play order
            _ => 0, // ONE-NINE: do nothing

        };
        return;
    }
}
