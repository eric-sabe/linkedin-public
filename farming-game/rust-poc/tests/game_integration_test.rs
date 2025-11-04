// Integration tests go here - the best tests!

use farming_game::models::{GameState, Player, PlayerType, TileEffect, AssetType};
use farming_game::game::GameEffect; // Added this import
use farming_game::cards::card::Card;
use farming_game::game::game_loop::handle_player_turn;
use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

// Helper to create a basic game state for integration tests
fn setup_integration_game(num_players: usize) -> GameState {
    let mut players = HashMap::new();
    let mut turn_order = Vec::new();
    for i in 0..num_players {
        let player_id = i;
        // Add players with some initial cash/debt different from default
        let mut player = Player::new(player_id, format!("Player {}", i+1), PlayerType::Human);
        player.cash = 7000 + (i as i32 * 1000); // Vary cash slightly
        player.debt = 4000 + (i as i32 * 500);  // Vary debt slightly
        // Add initial Hay/Grain given by Grandpa
        player.add_asset(AssetType::Hay, 10, 0);
        player.add_asset(AssetType::Grain, 10, 0);
        players.insert(player_id, player);
        turn_order.push(player_id);
    }
    // Use the real constructor to get decks etc., but provide our players
    GameState::new_with_players(players, turn_order) 
}

#[test]
fn test_basic_turn_integration() {
    let mut game_state = setup_integration_game(2);
    let player1_id = game_state.turn_order[0];
    let player2_id = game_state.turn_order[1];
    let mut logs: Vec<String> = Vec::new();

    // --- Player 1 Turn ---
    println!("\n--- Integration Test: Player 1 Turn ---");
    game_state.current_turn_index = 0;
    let initial_cash_p1 = game_state.players[&player1_id].cash;
    let player1_roll = 3; // Example roll
    
    // Call the actual turn handler
    let turn1_logs = handle_player_turn(&mut game_state, player1_id, player1_roll).unwrap();
    logs.extend(turn1_logs);

    assert_eq!(game_state.players[&player1_id].position, 3);
    assert_eq!(game_state.players[&player1_id].cash, 5000); // Starting cash
    // Verify Player 1 state (landed on Tile 4: Double Hay Yield)
    let player1 = game_state.players.get(&player1_id).unwrap();
    assert_eq!(player1.position, 4, "Player 1 ended on wrong tile");
    // Tile 4 effect (DoubleYieldForCrop) doesn't change cash directly
    assert_eq!(player1.cash, initial_cash_p1, "Player 1 cash incorrect after Tile 4"); 
    assert_eq!(player1.turns_taken, 1, "Player 1 turn count incorrect"); // Check turn count if relevant
    // Verify the multiplier was set
    assert_eq!(player1.get_crop_multiplier(&AssetType::Hay), 2.0, "Hay multiplier not set correctly by Tile 4");

    // --- Player 2 Turn ---
    println!("\n--- Integration Test: Player 2 Turn ---");
    game_state.current_turn_index = 1; // Manually advance turn index for test
    let initial_cash_p2 = game_state.players[&player2_id].cash;
    let initial_pos_p2 = game_state.players[&player2_id].position;
    let player2_roll = 5; // Example roll

    // Call the actual turn handler
    let turn2_logs = handle_player_turn(&mut game_state, player2_id, player2_roll).unwrap();
    logs.extend(turn2_logs);

    assert_eq!(game_state.players[&player2_id].position, 5);
    // Assuming tile 5 is a simple tile with no cash change
    // Verify Player 2 state (landed on Tile 6: Farmer's Fate)
    // We don't know the exact card drawn, so checks must be more general
    let player2 = game_state.players.get(&player2_id).unwrap();
    // Position could be 6 (if card had no move effect) or 2 (if Drought Year)
    assert!(
        player2.position == 6 || player2.position == 2,
        "Player 2 final position incorrect. Expected 6 or 2, got {}",
        player2.position
    );
    // Cash might change or not depending on card drawn and effects.
    // A simple check is that *something* happened or cash is unchanged.
    // We could potentially check the game log/player history if implemented.
    // For now, just check turn count.
    assert_eq!(player2.turns_taken, 1, "Player 2 turn count incorrect"); 

    // --- Player 1 Second Turn (Example of passing Go) ---
    println!("\n--- Integration Test: Player 1 Second Turn (Passing Go) ---");
    game_state.current_turn_index = 0; // Manually advance turn index
    let initial_cash_p1_t2 = game_state.players[&player1_id].cash;
    let initial_year_p1 = game_state.players[&player1_id].year;
    let player1_roll_t2 = 4;

    // Call the actual turn handler
    let turn3_logs = handle_player_turn(&mut game_state, player1_id, player1_roll_t2).unwrap();
    logs.extend(turn3_logs);

    assert_eq!(game_state.players[&player1_id].position, 7); // 3 + 4
    let player1_t2 = game_state.players.get(&player1_id).unwrap();
    assert_eq!(player1_t2.position, 0, "Player 1 ended on wrong tile after passing Go"); // Expected position is 0
    assert_eq!(player1_t2.year, initial_year_p1 + 1, "Player 1 year should advance after passing Go");
    // Expected cash: Start of Turn 2 cash + $5000 Pass Go bonus + $1000 Tile 0 bonus 
    // Note: Farmer's Fate card effect is random and handled separately if needed.
    let expected_cash_p1_t2 = initial_cash_p1_t2 + 5000 + 1000;
    assert_eq!(player1_t2.cash, expected_cash_p1_t2, "Player 1 cash incorrect after passing Go and landing on Tile 0");
    assert_eq!(player1_t2.turns_taken, 2, "Player 1 turn count incorrect on turn 2");
}

#[test]
fn test_dice_roll_distribution() {
    let mut rng = StdRng::from_entropy(); // Match game implementation
    let mut counts = [0; 6];
    let total_rolls = 100_000;

    // Perform rolls
    for _ in 0..total_rolls {
        let roll = rng.gen_range(1..=6);
        counts[roll - 1] += 1;
    }

    println!("\nDice Roll Distribution Test Results:");
    println!("Total Rolls: {}", total_rolls);
    println!("Expected count per number: {}", total_rolls / 6);
    println!("Allowed deviation: ±{}\n", (total_rolls / 6) / 5);  // 20% deviation

    println!("Actual distribution:");
    for (i, &count) in counts.iter().enumerate() {
        let percentage = (count as f64 / total_rolls as f64) * 100.0;
        println!("Roll {}: {} times ({:.2}%)", i + 1, count, percentage);
    }

    // Calculate chi-square statistic
    let expected = total_rolls / 6;
    let chi_square: f64 = counts.iter()
        .map(|&count| {
            let diff = count as f64 - expected as f64;
            (diff * diff) / expected as f64
        })
        .sum();

    println!("\nChi-square statistic: {}", chi_square);

    // Verify each count is within acceptable range (±20% of expected)
    let expected_count: usize = total_rolls / 6;
    let deviation = expected_count / 5;  // 20% of expected
    for &count in counts.iter() {
        assert!(
            count >= expected_count.saturating_sub(deviation) &&
            count <= expected_count + deviation,
            "Count {} is outside acceptable range ({} ± {})",
            count,
            expected_count,
            deviation
        );
    }

    // Chi-square test (5 degrees of freedom, p = 0.05)
    // Critical value is 11.07 at p = 0.05
    assert!(chi_square < 11.07, "Distribution is not uniform (chi-square = {})", chi_square);
} 