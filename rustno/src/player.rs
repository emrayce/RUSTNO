use crate::card::Card;
use std::fmt;

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

    pub fn can_play(&mut self, last_card: &Card) -> bool {
        for card in self.hand.iter_mut() {
            if card.same_color_as(last_card) | card.same_symbol_as(last_card) {
                return true;
            }
        }

        return false;
    }

    pub fn play_card(&mut self, card: usize) -> Card{
        self.hand.remove(card)
    }

    pub fn has_no_card(&mut self) -> bool {
        return self.hand.len() == 0;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.name)?;
        for card in self.hand.iter() {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}
