//! -------------------------------------------------------
//! Game of war as played by the rules defined here
//! https://www.pagat.com/war/war.html
//!

mod card;
mod deck;
mod hand;
mod player;
mod suit;

use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::Player;

// ------------------------------------------------------------------
struct Game {
    player1: Player,
    player2: Player,
}

/// Track the current state of the game.
#[derive(PartialEq)]
pub enum GameState {
    /// game is over, player 1 wins.
    Player1Wins,
    /// game is over, player 2 wins.
    Player2Wins,
    /// the infinite game, it's cycling.
    Tie,
    /// the game is still being played.
    StillPlaying,
}

impl Game {
    fn new() -> Game {
        // the cards in this deck are guarenteed to only be allocated
        // once for the entire game.  The cards can not be copied
        // because Copy hasn't been implemented for the Card type.
        let mut deck = Deck::new();

        deck.shuffle();

        let player1 = Player::new("player1", Hand::new(deck.deal(26)));
        let player2 = Player::new("player2", Hand::new(deck.deal(26)));

        Game { player1, player2 }
    }

    fn play_round(&mut self) -> GameState {
        // invarient: the number of cards from both hands add to 52.
        debug_assert_eq!(self.player1.count_cards() + self.player2.count_cards(), 52);

        // take a card from each player
        let card1 = self.player1.draw_card();
        let card2 = self.player2.draw_card();

        match (card1, card2) {
            // player1 ran out of cards.
            (None, _) => GameState::Player2Wins,
            // player2 ran out of cards.
            (_, None) => GameState::Player1Wins,
            (Some(c1), Some(c2)) => {
                if c1.rank == c2.rank {
                    self.declare_war(c1, c2)
                } else if c1.rank < c2.rank {
                    // player2 gets the cards
                    self.player2.gets_cards(vec![c1, c2]);
                    GameState::StillPlaying
                } else {
                    // player1 gets the cards
                    self.player1.gets_cards(vec![c1, c2]);
                    GameState::StillPlaying
                }
            }
        }
    }
    
    fn declare_war(&mut self, c1: Card, c2: Card) -> GameState {
        // make a pile to place the cards to win
        let mut pile: Vec<Card> = vec![c1, c2];

        loop {
            // draw 3 cards from player1
            if let Some(mut cards) = self.player1.draw(3) {
                pile.append(&mut cards)
            } else {
                // player1 ran out of cards.
                return GameState::Player2Wins;
            }

            // draw 3 cards from player2
            if let Some(mut cards) = self.player2.draw(3) {
                pile.append(&mut cards)
            } else {
                // player2 ran out of cards.
                return GameState::Player1Wins;
            }

            // now draw two more to see who wins.
            let card1: Option<Card> = self.player1.draw_card();
            let card2: Option<Card> = self.player2.draw_card();

            match (card1, card2) {
                (None, _) => {
                    // player1 runs out of cards.
                    return GameState::Player2Wins;
                }
                (_, None) => {
                    // player2 runs out of cards.
                    return GameState::Player1Wins;
                }
                (Some(c1), Some(c2)) => {
                    if c1.rank == c2.rank {
                        // another war!
                        pile.push(c1);
                        pile.push(c2);
                        continue;
                    } else if c1.rank < c2.rank {
                        // player2 wins the draw
                        pile.push(c1);
                        pile.push(c2);
                        self.player2.gets_cards(pile);
                        return GameState::StillPlaying;
                    } else {
                        // player1 wins the draw
                        pile.push(c1);
                        pile.push(c2);
                        self.player1.gets_cards(pile);
                        return GameState::StillPlaying;
                    }
                }
            }
        }
    }
}

fn play_game() -> GameState {
    let mut game = Game::new();
    let mut rounds_until_tie = 1000;

    while rounds_until_tie > 0 {
        let state = game.play_round();
        if state != GameState::StillPlaying {
            return state;
        }
        rounds_until_tie -= 1;
    }
    GameState::Tie
}

fn main() {
    let mut ties = 0;
    let mut player1 = 0;
    let mut player2 = 0;
    let num_games = 10000;

    for _ in 0..num_games {
        match play_game() {
            GameState::Tie => ties += 1,
            GameState::Player1Wins => player1 += 1,
            GameState::Player2Wins => player2 += 1,
            _ => {}
        }
    }

    println!(
        "SCORES: player1:{}, player2:{}, draws:{}",
        player1, player2, ties
    );
}
