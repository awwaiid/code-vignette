use crate::card::{Card, Rank};

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            total += card.value();
            if card.rank == Rank::Ace {
                aces += 1;
            }
        }

        // Adjust for aces if we're over 21
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }

        total
    }

    pub fn is_bust(&self) -> bool {
        self.value() > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn test_hand_value_simple() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Five));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Seven));
        assert_eq!(hand.value(), 12);
    }

    #[test]
    fn test_hand_value_with_ace() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Nine));
        assert_eq!(hand.value(), 20);
    }

    #[test]
    fn test_hand_value_ace_adjustment() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Five));
        hand.add_card(Card::new(Suit::Clubs, Rank::Seven));
        assert_eq!(hand.value(), 13); // Ace counts as 1
    }

    #[test]
    fn test_hand_bust() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::King));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Queen));
        hand.add_card(Card::new(Suit::Clubs, Rank::Five));
        assert!(hand.is_bust());
    }

    #[test]
    fn test_hand_blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Ace));
        hand.add_card(Card::new(Suit::Diamonds, Rank::King));
        assert!(hand.is_blackjack());
    }

    #[test]
    fn test_hand_not_blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Suit::Hearts, Rank::Seven));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Eight));
        hand.add_card(Card::new(Suit::Clubs, Rank::Six));
        assert_eq!(hand.value(), 21);
        assert!(!hand.is_blackjack());
    }
}