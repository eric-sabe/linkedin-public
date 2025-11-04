// Consolidated config file for the Farming Game
// This replaces the individual modules in the config folder

use crate::models::PlayerType;

//----------------------------------------
// Game Rules (from game_rules.rs)
//----------------------------------------

// Winning condition
pub const WINNING_NET_WORTH: i32 = 250_000;

//----------------------------------------
// Player Configuration (from player_config.rs)
//----------------------------------------

#[derive(Clone)]
pub struct NativePlayer {
    pub name: &'static str,
    pub color: &'static str,
}

pub const NATIVE_PLAYERS: [NativePlayer; 6] = [
    NativePlayer { name: "Roza Ray", color: "Red" },
    NativePlayer { name: "Harrah Harry", color: "Brown" },
    NativePlayer { name: "Toppenish Tom", color: "Green" },
    NativePlayer { name: "Satus Sam", color: "Blue" },
    NativePlayer { name: "Sunnyside Sidney", color: "White" },
    NativePlayer { name: "Wapato Willie", color: "Yellow" },
];

pub const STARTING_CASH: i32 = 5000;
pub const STARTING_LAND: i32 = 20;  // 20 acres from Grandpa
pub const STARTING_DEBT: i32 = 0;
pub const STARTING_YEAR: u32 = 1;
pub const STARTING_POSITION: usize = 0;  // Kept this as it's used in Player::new()

pub fn create_ai_player(name: &str) -> PlayerType {
    PlayerType::AI(name.to_string())
}
