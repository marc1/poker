mod game;

#[cfg(test)]
mod test {
    use crate::game::{Card,Rank,Suit};
    
    #[test]
    fn bruh() {
        let x = Card::new(Rank::Ace, Suit::Spade);
        let y = Card::new(Rank::Ace, Suit::Heart);
    }
}
