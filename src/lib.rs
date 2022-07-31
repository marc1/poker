#![allow(warnings)]

use std::fmt::Display;

#[derive(Debug)]
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

impl From<Rank> for u8 {
    fn from(value: Rank) -> Self {
        value as u8
    }
}

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value % 13) }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl From<Suit> for u8 {
    fn from(value: Suit) -> u8 {
        value as u8
    }
}

impl From<u8> for Suit {
    fn from(value: u8) -> Suit {
        unsafe { std::mem::transmute(value % 4) }
    }
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
    use crate::Rank;

    #[test]
    fn foo() {
        let mut v: Vec<Rank> = vec![];
        for i in 0..52 {
            let r: Rank = Rank::from(i);
            v.push(r);
        }
    }
}
