use crate::card::*;

// ------------------------------------------------------------------
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand{cards}
    }
    
    pub fn has_cards(&self) -> bool {
        !self.cards.is_empty()
    }

    pub fn count_cards(&self) -> usize {
        self.cards.len()
    }
    
    pub fn draw(&mut self, n: usize) -> Option<Vec<Card>> {
        if self.cards.len() < n {
            None
        } else {
            Some(self.cards.drain(0..n).collect())
        }
    }
    
    pub fn gets_cards(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }
}

// ------------------------------------------------------------------

