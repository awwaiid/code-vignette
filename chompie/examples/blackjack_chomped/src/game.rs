use crate::deck::Deck;
use crate::hand::Hand;

pub enum GameResult {
    PlayerWins,
    DealerWins,
    Push,
}

pub struct Game {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let mut player_hand = Hand::new();
        let mut dealer_hand = Hand::new();

        // Deal initial cards
        player_hand.add_card(deck.draw().unwrap());
        dealer_hand.add_card(deck.draw().unwrap());
        player_hand.add_card(deck.draw().unwrap());
        dealer_hand.add_card(deck.draw().unwrap());

        Game {
            deck,
            player_hand,
            dealer_hand,
        }
    }

    pub fn player_hit(&mut self) {
        if let Some(card) = self.deck.draw() {
            self.player_hand.add_card(card);
        }
    }

    pub fn dealer_play(&mut self) {
        while self.dealer_hand.value() < 17 {
            if let Some(card) = self.deck.draw() {
                self.dealer_hand.add_card(card);
            } else {
                break;
            }
        }
    }

    pub fn result(&self) -> GameResult {
        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();

        if self.player_hand.is_bust() {
            return GameResult::DealerWins;
        }

        if self.dealer_hand.is_bust() {
            return GameResult::PlayerWins;
        }

        if player_value > dealer_value {
            GameResult::PlayerWins
        } else if dealer_value > player_value {
            GameResult::DealerWins
        } else {
            GameResult::Push
        }
    }

    pub fn player_hand(&self) -> &Hand {
        &self.player_hand
    }

    pub fn dealer_hand(&self) -> &Hand {
        &self.dealer_hand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(game.player_hand().cards().len(), 2);
        assert_eq!(game.dealer_hand().cards().len(), 2);
    }

    #[test]
    fn test_player_hit() {
        let mut game = Game::new();
        game.player_hit();
        assert_eq!(game.player_hand().cards().len(), 3);
    }

    #[test]
    fn test_dealer_play() {
        let mut game = Game::new();
        game.dealer_play();
        // Dealer should have at least 2 cards and value >= 17 or bust
        assert!(game.dealer_hand().cards().len() >= 2);
        let value = game.dealer_hand().value();
        assert!(value >= 17 || game.dealer_hand().is_bust());
    }
}