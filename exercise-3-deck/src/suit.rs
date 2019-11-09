
#[derive(Clone, PartialEq, Copy)]
pub enum Suit {
    Diamond,
    Club,
    Spade,
    Heart,
}

// `pub use ...` to export enum constructors.
pub use Suit::*;

impl Suit {
    pub fn to_num(&self) -> usize {
        match *self {
            Spade => 0,
            Heart => 1,
            Diamond => 2,
            Club => 3,
        }
    }
}


// ------------------------------------------------------------------
// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    
    fn test_suit() {
        // not a great set of tests, but ok!
        assert_eq!(Suit::Spade.to_num(), 0);
        assert_eq!(Suit::Heart.to_num(), 1);
        assert_eq!(Suit::Diamond.to_num(), 2);
        assert_eq!(Suit::Club.to_num(), 3);
    }
}
