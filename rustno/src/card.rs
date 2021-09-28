enum Symbol {
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

enum Color {
    RED,
    YELLOW,
    BLUE,
    GREEN,
    BLACK,
}

struct Card {
    symbol: Symbol,
    color: Color,
    // Effect
    // ONE-NINE: do nothing
    // STOP: next player can't play,
    // PLUS_2: NEXT PLAYER draw 2 cards
    // PLUS_4: NEXT PLAYER draw 4 cards and current player chose a new color
    // CHANGE_COLOR: Current player chose a new color
    // CHANGE_DIRECTION: Inverse the play order
}
