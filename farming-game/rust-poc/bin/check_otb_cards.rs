use farming_game::models::{GameState, TileType};
use farming_game::cards::card::Card;
use farming_game::game::GameEffect;
use farming_game::config::NATIVE_PLAYERS;
use std::collections::HashSet;

fn main() {
    println!("OTB Card Distribution Diagnostic");
    println!("===============================\n");

    // Create a simple game with all players
    let mut game = GameState::new();
    
    // Print initial deck status
    println!("Option to Buy Deck state:");
    println!("  Draw pile: {} cards", game.option_to_buy_deck.draw_pile.len());
    println!("  Discard pile: {} cards\n", game.option_to_buy_deck.discard_pile.len());

    // Initialize player hands with OTB cards manually, like main.rs does
    println!("Manually distributing cards to players...");
    for player_id in 0..NATIVE_PLAYERS.len() {
        if let Some(player) = game.players.get(&player_id) {
            println!("\nDistributing OTB cards to {}", player.name);
            
            for i in 0..2 {
                match game.draw_card(TileType::OptionToBuy) {
                    Ok(card) => {
                        // Note: Using unwrap_or_else to handle the case when player might not exist
                        game.players.get_mut(&player_id).unwrap().hand.push(card.clone());
                        println!("  Card {}: {} - {}", i + 1, card.title, card.description);
                    }
                    Err(e) => {
                        println!("  Error drawing card: {}", e);
                        if !game.option_to_buy_deck.discard_pile.is_empty() {
                            println!("  Reshuffling discard pile...");
                            game.option_to_buy_deck.draw_pile = game.option_to_buy_deck.discard_pile.clone();
                            game.option_to_buy_deck.discard_pile.clear();
                            game.option_to_buy_deck.shuffle();
                            
                            if let Ok(card) = game.draw_card(TileType::OptionToBuy) {
                                game.players.get_mut(&player_id).unwrap().hand.push(card.clone());
                                println!("  Card {}: {} - {}", i + 1, card.title, card.description);
                            }
                        }
                    }
                }
            }
        }
    }

    // Verify the OTB card distribution
    println!("\n==== VERIFYING CARD DISTRIBUTION ====");
    for player_id in 0..NATIVE_PLAYERS.len() {
        if let Some(player) = game.players.get(&player_id) {
            // Get all cards and then count OTB cards specifically, including BOTH types of OTB cards
            let otb_cards: Vec<&Card> = player.hand.iter()
                .filter(|card| matches!(card.effect, 
                    GameEffect::OptionalBuyAsset { .. } | 
                    GameEffect::LeaseRidge { .. }
                ))
                .collect();
            
            println!("{} has {} total cards ({} OTB cards):", 
                player.name, player.hand.len(), otb_cards.len());
            
            for (i, card) in otb_cards.iter().enumerate() {
                match &card.effect {
                    GameEffect::OptionalBuyAsset { .. } => {
                        println!("  OTB Card {}: {} - {}", i + 1, card.title, card.description);
                    },
                    GameEffect::LeaseRidge { .. } => {
                        println!("  OTB Card {}: {} - {} (Ridge card)", i + 1, card.title, card.description);
                    },
                    _ => {
                        println!("  OTB Card {}: {} - {} (Unknown type)", i + 1, card.title, card.description);
                    }
                }
            }
            
            // If player doesn't have exactly 2 OTB cards, flag as an issue
            if otb_cards.len() != 2 {
                println!("  WARNING: Player should have 2 OTB cards but has {}!", otb_cards.len());
            }
        }
    }

    // Check if we've exhausted the OTB deck
    println!("\nFinal OTB deck state:");
    println!("  Draw pile: {} cards", game.option_to_buy_deck.draw_pile.len());
    println!("  Discard pile: {} cards", game.option_to_buy_deck.discard_pile.len());
    
    if game.option_to_buy_deck.draw_pile.is_empty() && game.option_to_buy_deck.discard_pile.is_empty() {
        println!("  WARNING: OTB deck completely empty! May not have enough cards for all players.");
    }
} 