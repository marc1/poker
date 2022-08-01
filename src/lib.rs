#![allow(warnings)]

use std::{cmp::Ordering, fmt::{self, Display}};

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

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", match self {
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Ten => 'T',
            Self::Jack => 'J',
            Self::Queen => 'Q',
            Self::King => 'K',
            Self::Ace => 'A'
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", match self {
            Self::Club => 'c',
            Self::Diamond => 'd',
            Self::Heart => 'h',
            Self::Spade => 's',
        })
    }
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

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.rank(), self.suit())
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
            println!("{x}");
        }
    }
}
