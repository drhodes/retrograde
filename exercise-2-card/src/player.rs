
use crate::card::*;
use crate::hand::*;

// ------------------------------------------------------------------
pub struct Player {
    pub name: &'static str,    
    pub hand: Hand,
}

impl Player {
    pub fn new(name: &'static str, hand: Hand) -> Player {
        // name is reference to a str with static lifetime, that is,
        // this str will exist for the entire runtime of the process.        
        Player { name, hand }
    }
    
    pub fn draw_card(&mut self) -> Option<Card> {
        // Draw a card. This removes a card from the hand.
        if self.hand.has_cards() {
            
            // The methods receiver (self) is declared &mut because
            // the following call to `remove` mutates the player's hand
            // by popping the first card. We're moving the card here
            // from self.hand to whomever called this function.
            Some(self.hand.cards.remove(0))
                
        } else {
            // no cards left in the hand!
            None
        }
    }

    pub fn draw(&mut self, n: usize) -> Option<Vec<Card>> {
        self.hand.draw(n)
    }
    
    pub fn gets_cards(&mut self, cards: Vec<Card>) {
        self.hand.gets_cards(cards);
        // invarient: cards will be empty.
    }

    pub fn count_cards(&self) -> usize {
        self.hand.count_cards()
    }

}


// ------------------------------------------------------------------
// TESTS

#[cfg(test)]
mod tests {
}
