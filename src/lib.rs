#![allow(warnings)]

use std::{cmp::Ordering, fmt::Display};

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
    use crate::{Card, Rank, Suit};

    #[test]
    fn card_cmp() {
        let c1 = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        let c2 = Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        };

        let c3 = Card {
            rank: Rank::Ten,
            suit: Suit::Spade,
        };

        assert!(c1 > c2);
        assert!(c2 > c3);
    }

    #[test]
    fn deck() {
        let deck: Vec<Card> = (0u8..52)
            .map(|i| Card {
                rank: unsafe { std::mem::transmute(i / 4) },
                suit: unsafe { std::mem::transmute(i % 4) },
            })
            .collect();

        for x in deck {
            println!("{x:?}");
        }
    }
}
