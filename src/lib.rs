#![allow(warnings)]

use std::{
    cmp::Ordering,
    fmt::Display
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank() == other.rank() {
            self.suit().cmp(&other.suit())
        } else {
            self.rank().cmp(&other.rank())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rank, Suit, Card};

    #[test]
    fn card_cmp() {
        let c1 = Card { rank: Rank::Ace, suit: Suit::Spade };
        let c2 = Card { rank: Rank::Ace, suit: Suit::Club };

        let c3 = Card { rank: Rank::Ten, suit: Suit::Spade };

        assert!(c1 > c2);
        assert!(c2 > c3);
    }
}
