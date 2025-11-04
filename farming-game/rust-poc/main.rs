// Farming Game Remake - Core Game State and Setup (Rust)

mod models;
mod game;
mod cards;
mod config;
// mod ui; // Removed - now declared in lib.rs

use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use farming_game::models::{Player, PlayerType, GameState, TileType};
use farming_game::game::GameEffect; // Add GameEffect import
use farming_game::cards::card::Card; // Add Card import
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use farming_game::config::NATIVE_PLAYERS; // Updated import path
use farming_game::config::{STARTING_CASH, STARTING_DEBT, STARTING_LAND, STARTING_POSITION, STARTING_YEAR}; // Added constants
use farming_game::ui::terminal; // Import terminal functions
use farming_game::ui::app::App; // Import the App struct
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { // Return Result for error handling
    // 1. Setup Game State (before initializing TUI)
    let game_state = setup_game()?; // Call setup function

    // 2. Initialize terminal
    let mut tui = terminal::init()?;

    // 3. Create and run the UI application, passing the initialized state
    let mut app = App::new(game_state); // Pass game_state to App::new
    app.run(&mut tui)?; // Run the main TUI loop

    // 4. Restore terminal before exiting
    terminal::restore()?;
    Ok(())
}

/// Sets up the initial GameState by interacting with the user.
fn setup_game() -> Result<GameState, Box<dyn Error>> {
    // --- Logic moved from original main --- 
    println!("Welcome to the Farming Game!");
    print!("Enter number of players (3-6) [default: 3]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let num_players: usize = if input.trim().is_empty() {
        3
    } else {
        match input.trim().parse() {
            Ok(n) if n >= 3 && n <= 6 => n,
            _ => {
                println!("Invalid input. Using default of 3 players.");
                3
            }
        }
    };

    let mut players = HashMap::new();
    let mut turn_order = Vec::new();
    thread::sleep(Duration::from_millis(100));
    let mut available_native_players = NATIVE_PLAYERS.to_vec();
    available_native_players.shuffle(&mut rand::thread_rng());

    for i in 0..num_players {
        let native_player = &available_native_players[i];
        print!("Enter nickname for {} ({}) [default: {}]: ", native_player.name, native_player.color, native_player.color);
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let nickname = if input.trim().is_empty() {
            native_player.color.to_string()
        } else {
            input.trim().to_string()
        };
        let display_name = format!("{} ({})", native_player.name, nickname);

        players.insert(i, Player {
            id: i,
            name: display_name,
            player_type: PlayerType::Human,
            cash: STARTING_CASH,
            debt: STARTING_DEBT,
            land: STARTING_LAND,
            is_active: true,
            position: STARTING_POSITION,
            year: STARTING_YEAR,
            eligible_for_side_job_pay: true,
            crop_yield_multipliers: HashMap::new(),
            assets: HashMap::new(), // Start with no explicit assets, handled later if needed
            history: vec![],
            completed_harvests: HashSet::new(),
            persistent_effects: vec![],
            hand: vec![],
            active_persistent_cards: vec![],
            net_worth: 0, // Will be calculated by GameState::new_with_players
            total_asset_value: 0,
            total_ridge_value: 0,
            total_income: 0,
            total_expenses: 0,
            turns_taken: 0,
        });
        turn_order.push(i);
    }

    let mut game = GameState::new_with_players(players, turn_order);

    println!("\nInitial Deck Sizes:");
    println!("Farmer's Fate Deck: {} cards", game.farmer_fate_deck.draw_pile.len());
    println!("Operating Cost Deck: {} cards", game.operating_cost_deck.draw_pile.len());
    println!("Option to Buy Deck: {} cards", game.option_to_buy_deck.draw_pile.len());

    for player_id in 0..num_players {
        let player_name = game.players[&player_id].name.clone();
        println!("\nGiving {} their initial Option to Buy cards...", player_name);
        for i in 0..2 {
            match game.draw_card(TileType::OptionToBuy) {
                Ok(card) => {
                    game.players.get_mut(&player_id).unwrap().hand.push(card.clone());
                    println!("  Card {}: {} - {}", i + 1, card.title, card.description);
                }
                Err(e) => {
                    println!("Error drawing card for {}: {}", player_name, e);
                    if !game.option_to_buy_deck.discard_pile.is_empty() {
                        println!("Reshuffling discard pile...");
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

    // Verify the final OTB card distribution
    println!("\n==== VERIFYING CARD DISTRIBUTION ====");
    for player_id in 0..num_players {
        let player = &game.players[&player_id];
        let otb_cards: Vec<&Card> = player.hand.iter()
            .filter(|card| matches!(card.effect, GameEffect::OptionalBuyAsset { .. }))
            .collect();
        
        println!("{} has {} total cards ({} OTB cards):", 
            player.name, player.hand.len(), otb_cards.len());
        
        for (i, card) in otb_cards.iter().enumerate() {
            println!("  OTB Card {}: {} - {}", i + 1, card.title, card.description);
        }
        
        // If OTB cards != 2, highlight this as an issue
        if otb_cards.len() != 2 {
            println!("  WARNING: {} should have 2 OTB cards but has {}!", 
                player.name, otb_cards.len());
        }
    }

    Ok(game) // Return the initialized GameState
}

