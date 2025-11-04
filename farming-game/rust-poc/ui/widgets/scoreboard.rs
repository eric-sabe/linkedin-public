// src/ui/widgets/scoreboard.rs

use ratatui::{
    prelude::{Constraint, Rect, Frame},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table},
};
use crate::models::{GameState, Player, asset::AssetType}; // Import Player and AssetType
 // For formatting strings

/// Renders the scoreboard widget.
pub fn render_scoreboard(frame: &mut Frame, area: Rect, game_state: &GameState) {
    // Create header with columns for each important stat
    let header_cells = [
        "Player", "Cash", "Debt", "Net Worth", 
        "Grain", "Hay", "Cows", "Fruit", 
        "Tractor", "Harvester", "Pos", "Turn", "Year"
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).bold()));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);

    // Get the current player ID for highlighting
    let current_player_id = game_state.turn_order[game_state.current_turn_index];

    // Generate rows from game_state data
    let rows: Vec<Row> = game_state.turn_order.iter().map(|player_id| {
        let player = game_state.players.get(player_id).expect("Player ID in turn_order not found");

        // Format crop quantities with multipliers
        let grain_cell = format_asset_cell(player, AssetType::Grain);
        let hay_cell = format_asset_cell(player, AssetType::Hay);
        let cows_cell = format_asset_cell(player, AssetType::Cows);
        let Fruit_cell = format_asset_cell(player, AssetType::Fruit);
        
        // Equipment yes/no values
        let has_tractor = if player.assets.contains_key(&AssetType::Tractor) { "Yes" } else { "No" };
        let has_harvester = if player.assets.contains_key(&AssetType::Harvester) { "Yes" } else { "No" };

        let row = Row::new(vec![
            Cell::from(player.name.clone()),
            Cell::from(format!("${}", player.cash)),
            Cell::from(format!("${}", player.debt)),
            Cell::from(format!("${}", player.net_worth)),
            Cell::from(grain_cell),
            Cell::from(hay_cell),
            Cell::from(cows_cell),
            Cell::from(Fruit_cell),
            Cell::from(has_tractor.to_string()),
            Cell::from(has_harvester.to_string()),
            Cell::from(player.position.to_string()),
            Cell::from(player.turns_taken.to_string()),
            Cell::from(player.year.to_string()),
        ]);

        // Highlight the current player's row
        if *player_id == current_player_id {
            row.style(Style::default().bg(Color::Blue))
        } else {
            row
        }
    }).collect();

    // Define column widths
    let widths = [
        Constraint::Length(25), // Player Name (increased width)
        Constraint::Length(8),  // Cash
        Constraint::Length(8),  // Debt
        Constraint::Length(10), // Net Worth
        Constraint::Length(8),  // Grain
        Constraint::Length(8),  // Hay
        Constraint::Length(8),  // Cows
        Constraint::Length(8),  // Fruit
        Constraint::Length(7),  // Tractor
        Constraint::Length(9),  // Harvester
        Constraint::Length(4),  // Pos
        Constraint::Length(5),  // Turn
        Constraint::Length(4),  // Year
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Scoreboard"))
        .column_spacing(1);

    frame.render_widget(table, area);
}

/// Helper function to format asset cell with quantity and multiplier
fn format_asset_cell(player: &Player, asset_type: AssetType) -> String {
    let quantity = player.assets.get(&asset_type).map_or(0, |record| record.quantity);
    let multiplier = player.get_crop_multiplier(&asset_type);
    
    if multiplier == 1.0 {
        format!("{}", quantity)
    } else {
        format!("{}×{:.1}", quantity, multiplier)
    }
} 