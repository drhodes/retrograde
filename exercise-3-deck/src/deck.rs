#![allow(warnings, missing_docs)]

use crate::card::{Card, RANK_HIGH, RANK_LOW};
use crate::suit::{Club, Diamond, Heart, Spade};
use rand::seq::SliceRandom;

// ------------------------------------------------------------------
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        // TODO: create and return a full deck of cards.
        Deck{ cards: vec!() }
    }

    pub fn shuffle(&mut self) -> () {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        // panic if num_cards > self.cards.len
        self.cards.split_off(num_cards)

        // Is this a good design decision? Should the whole program
        // halt if Deck::deal is called with an out of range n?
    }
}


// ------------------------------------------------------------------
// TESTS 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deck_constructor() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52, "Deck should have 52 cards");        
    }
     
    #[test]
    fn deck_suits() {
        let deck = Deck::new();
        let mut num_hearts = 0;
        let mut num_spades = 0;
        let mut num_diamonds = 0;
        let mut num_clubs = 0;
        
        for c in deck.cards {
            match c.suit {
                Heart => num_hearts += 1,
                Club => num_clubs += 1,
                Diamond => num_diamonds += 1,
                Spade => num_spades += 1,
            }
        }

        assert_eq!(num_hearts, 13, "wrong number of hearts in new deck");
        assert_eq!(num_spades, 13, "wrong number of spades in new deck");
        assert_eq!(num_diamonds, 13, "wrong number of diamonds in new deck");
        assert_eq!(num_clubs, 13, "wrong number of clubs in new deck");
    }

    #[test]
    fn deck_ranks() {
        let deck = Deck::new();
        let mut total_rank = 0;
        for c in deck.cards {
            total_rank += c.rank;
        }
        
        const ALL_RANKS_SUMMED: usize = 416;
        assert_eq!(total_rank, ALL_RANKS_SUMMED);
    }
}
