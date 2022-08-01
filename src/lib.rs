#![allow(warnings)]

use std::fmt::Display;

enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn rank(self) -> Rank {
        self.rank
    }

    fn suit(self) -> Suit {
        self.suit
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rank, Suit, Card};

    #[test]
    fn foo() {

    }
}
