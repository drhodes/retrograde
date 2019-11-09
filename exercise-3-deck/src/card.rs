use std::fmt;

use crate::suit::*;

pub type Rank = usize;

pub const RANK_LOW: Rank = 2;
pub const RANK_HIGH: Rank = 14;

#[derive(PartialEq)]
pub struct Card {
    /// rank is a number of type usize from 2 to 14 where 14 is rank
    /// for Ace
    pub rank: Rank,
    
    /// suit is one of {space, heart, club, diamond}, you should be
    /// able to cursor over Suit and $goto-definition to see how the
    /// Suit type is declared.
    pub suit: Suit,
}

const ALL_CARDS: &str = "🂢🂲🃂🃒🂣🂳🃃🃓🂤🂴🃄🃔🂥🂵🃅🃕🂦🂶🃆🃖🂧🂷🃇🃗🂨🂸🃈🃘🂩🂹🃉🃙🂪🂺🃊🃚🂫🂻🃋🃛🂭🂽🃍🃝🂮🂾🃎🃞🂡🂱🃁🃑";

impl Card {
    /// Construct a card from a rank and suit. If a rank value is
    /// passed in that is out of range then the program should panic!

    pub fn new(rank: Rank, suit: Suit) -> Card {
        if rank < RANK_LOW || rank > RANK_HIGH {
            panic!("Rank is out of range: {}", rank);
        }
        return Card { rank, suit };
    }
    
    pub fn show(&self) -> String {
        let idx = ((self.rank - 2) * 4) + self.suit.to_num();
        if let Some(c) = ALL_CARDS.chars().nth(idx) {
            return c.to_string();
        } else {
            panic!("Card::show can't build a string representation of this card");
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.show())
    }
}


// ------------------------------------------------------------------
// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn bad_card0() {
        Card::new(0, Suit::Spade);
        Card::new(1, Suit::Spade);
    }
    
    #[test]
    fn good_cards() {
        for i in 2..14+1 {
            Card::new(i, Suit::Spade);
        }
    }
    #[test]
    #[should_panic]
    fn bad_card3() {
        for i in 15..10000 {
            // lots of panicing.
            Card::new(i, Suit::Heart);
        }
    }
}
