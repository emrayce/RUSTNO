use crate::card;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<card::Card>,
}

impl Player {

    pub fn new(name: String) -> Player {
        Player { name, hand: Vec::new() }
    }

    pub fn draw(&mut self, mut deck: Vec<card::Card>, n: usize) {
        let tmp = deck.pop();

        match tmp {
            Some(x) => self.hand.push(x),
            None => (),
        };
    }
}
