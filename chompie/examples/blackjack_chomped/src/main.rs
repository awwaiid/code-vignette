mod card;
mod deck;
mod hand;
mod game;

use game::{Game, GameResult};

fn main() {
    println!("Starting Blackjack Game!");

    let mut game = Game::new();

    println!("\nPlayer hand: {:?}", game.player_hand().value());
    println!("Dealer showing: {:?}", game.dealer_hand().cards()[0]);

    // Simple AI: hit if below 17
    while game.player_hand().value() < 17 {
        game.player_hit();
        println!("Player hits! New value: {}", game.player_hand().value());
    }

    if game.player_hand().is_bust() {
        println!("Player busts!");
    } else {
        println!("Player stands at {}", game.player_hand().value());
    }

    game.dealer_play();
    println!("Dealer final value: {}", game.dealer_hand().value());

    match game.result() {
        GameResult::PlayerWins => println!("Player wins!"),
        GameResult::DealerWins => println!("Dealer wins!"),
        GameResult::Push => println!("Push!"),
    }
}
