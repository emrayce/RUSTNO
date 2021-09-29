use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {

    pub fn new(name: String) -> Player {
        Player { name, hand: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }
}
