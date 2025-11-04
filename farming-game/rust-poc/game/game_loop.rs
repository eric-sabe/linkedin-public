// src/game/game_loop.rs

use crate::models::GameState;

// Change function signature to return logs or an error string
pub fn handle_player_turn(game: &mut GameState, player_id: usize, roll: u32) -> Result<Vec<String>, String> {
    let mut turn_logs: Vec<String> = Vec::new();

    // --- 1. Get immutable info ---
    let _player_name = game.players.get(&player_id)
        .ok_or_else(|| format!("Player with ID {} not found.", player_id))?
        .name.clone();
    let old_position = game.players.get(&player_id)
        .ok_or_else(|| format!("Invalid player ID: {}", player_id))?
        .position;
    let board_len = game.board.len(); 
    let total_pos = old_position + roll as usize;
    let new_position = total_pos % board_len;
    let current_tile = game.board.get(new_position)
        .ok_or_else(|| format!("Invalid board position: {}", new_position))?
        .clone();

    // --- 2. Handle Passing Go and Move Player ---
    {
        let player = game.players.get_mut(&player_id)
             .ok_or_else(|| format!("Invalid player ID: {}", player_id))?;

        // Increment turns taken
        player.turns_taken += 1;
        
        if old_position + roll as usize >= board_len {
            turn_logs.push(format!("{} passed Go (Tile 0)!", player.name));

            player.year += 1;
            turn_logs.push(format!("Year advanced to {}.", player.year));

            if player.eligible_for_side_job_pay {
                player.cash += 5000;
                turn_logs.push(format!("Collected $5000 side job pay. Cash: ${}", player.cash));
            } else {
                turn_logs.push("Did not collect side job pay (ineligible this year).".to_string());
            }
            player.eligible_for_side_job_pay = true;
            player.reset_crop_multipliers();
        }

        // Move player
        player.position = new_position;
    } 

    // --- 3. Handle Tile Effects & Harvest ---
    turn_logs.push(format!("Rolled a {} - landed on {}", 
        roll, 
        current_tile.name.as_str()
    ));
    
    // Only show tile description if it's meaningful
    if let Some(desc) = &current_tile.description {
        if !desc.is_empty() {
            turn_logs.push(desc.clone());
        }
    }
    
    // Call handle_tile_event, passing mutable logs
    if let Err(e) = game.handle_tile_event(player_id, &current_tile, &mut turn_logs) {
         // Log error from primary tile effect handling
         turn_logs.push(format!("Error handling tile event: {}", e));
    }

    // --- 4. Display Summaries (Removed - handled by TUI) ---
    /* (Commented out summary section)
    {
        let player = game.players.get(&player_id)
            .ok_or_else(|| format!("Invalid player ID: {}", player_id))?;
        // ... all println! for summaries ...
    }
    */

    // --- 5. Option To Buy loop (Needs Modification for TUI input/output) ---
    // TODO: Refactor OTB loop for TUI interaction (e.g., return required info to App)
    // For now, comment it out to avoid blocking/println!
    /*
    let mut input = String::new();
    loop {
        // ... existing OTB logic using println! and read_line ...
    }
    */

    // --- 6. Update Player Scoreboard (Done implicitly by game state changes) ---
    // Ensure scoreboard data is updated within the game logic where changes occur
    if let Some(player) = game.players.get_mut(&player_id) {
        player.update_scoreboard();
    }
    
    // Return the accumulated logs for this turn
    Ok(turn_logs)
} 