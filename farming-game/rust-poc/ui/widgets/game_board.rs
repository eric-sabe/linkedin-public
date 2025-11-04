use ratatui::{
    prelude::{Rect, Frame, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    layout::Alignment,
};
use crate::models::{GameState, HarvestType};
use std::collections::HashMap;

// Define colors for the players on the board
const PLAYER_COLORS: [Color; 6] = [
    Color::Blue,
    Color::Green,
    Color::Red,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
];

// Helper function to get player color
fn get_player_color(player_id: usize) -> Color {
    PLAYER_COLORS[player_id % PLAYER_COLORS.len()]
}

// Helper function to get harvest color
fn get_harvest_color(harvest_type: &HarvestType) -> Option<Color> {
    match harvest_type {
        HarvestType::HayCutting1 | HarvestType::HayCutting2 | 
        HarvestType::HayCutting3 | HarvestType::HayCutting4 => Some(Color::Rgb(144, 238, 144)), // Light Green
        HarvestType::Cherry | HarvestType::Apple => Some(Color::Red),
        HarvestType::Wheat | HarvestType::Corn => Some(Color::Yellow),
        HarvestType::Livestock => Some(Color::Rgb(205, 133, 63)), // Sandy Brown - more distinct from red
        HarvestType::None => None,
    }
}

// Helper function to get harvest symbol
fn get_harvest_symbol(harvest_type: &HarvestType) -> &'static str {
    match harvest_type {
        HarvestType::HayCutting1 | HarvestType::HayCutting2 | 
        HarvestType::HayCutting3 | HarvestType::HayCutting4 => "H",
        HarvestType::Cherry | HarvestType::Apple => "F",
        HarvestType::Wheat | HarvestType::Corn => "G",
        HarvestType::Livestock => "L",
        HarvestType::None => "",
    }
}

// Helper function to create a shortened tile name
fn get_short_tile_name(tile_name: &str) -> String {
    // Try to extract month and week information
    let parts: Vec<&str> = tile_name.split_whitespace().collect();
    
    if parts.len() >= 2 {
        // Check if it's a month+week format (e.g., "January Week 1")
        if parts[1] == "Week" && parts.len() >= 3 {
            // It's in format "Month Week Number"
            if parts[0].len() >= 3 {
                return format!("{} W{}", &parts[0][..3], parts[2]);
            }
        }
    }
    
    // For special tiles or tiles without standard naming
    if tile_name.len() > 10 {
        tile_name[..10].to_string()
    } else {
        tile_name.to_string()
    }
}

/// Renders the game board with player positions using ratatui Layout.
pub fn render_game_board(frame: &mut Frame, area: Rect, game_state: &GameState) {
    let board_block = Block::default()
        .title("Game Board")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    // Render the outer block first
    frame.render_widget(board_block.clone(), area);

    // Get the inner area for the grid layout
    let inner_area = board_block.inner(area);

    // --- Player Position Mapping ---
    let mut players_by_position: HashMap<usize, Vec<usize>> = HashMap::new();
    for player_id in &game_state.turn_order {
        let player = &game_state.players[player_id];
        if player.is_active {
            players_by_position
                .entry(player.position) // Use direct position
                .or_insert_with(Vec::new)
                .push(*player_id);
        }
    }

    // --- Layout Definition ---
    // Define the 4 rows using fixed length for consistent height
    let row_constraints = [
        Constraint::Length(4), // Row 0
        Constraint::Length(4), // Row 1
        Constraint::Length(4), // Row 2
        Constraint::Length(4), // Row 3
    ];
    let row_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(inner_area);

    // Define plots per row - Collect into Vecs first to avoid lifetime issues
    let row0_plots: Vec<usize> = (0..=13).collect();  // Row 0: 14 plots
    let row1_plots: Vec<usize> = (14..=24).collect(); // Row 1: 11 plots
    let row2_plots: Vec<usize> = (25..=36).collect(); // Row 2: 12 plots (Ends before 37)
    let row3_plots: Vec<usize> = (37..=48).collect(); // Row 3: 12 plots (Starts with 37)

    let plots_in_row: [&[usize]; 4] = [
        &row0_plots,
        &row1_plots,
        &row2_plots,
        &row3_plots,
    ];

    // --- Render Each Row ---
    let max_plots_in_row = 14; // Based on row 2 (25-38)

    for (row_index, row_area) in row_layout.iter().enumerate() {
        let current_plots = plots_in_row[row_index];
        let num_plots_in_this_row = current_plots.len();

        // Create constraints for a fixed number of columns based on the max plots in any row
        let plot_constraints = std::iter::repeat(Constraint::Ratio(1, max_plots_in_row as u32))
                                .take(max_plots_in_row)
                                .collect::<Vec<_>>();

        let plot_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(plot_constraints)
            .split(*row_area);

        // --- Render Each Plot in the Row (up to the actual number of plots for this row) ---
        for (col_index, plot_area) in plot_layout.iter().enumerate() {
            // Only render if the column index corresponds to an actual plot in this specific row
            if col_index < num_plots_in_this_row {
                if let Some(&plot_index) = current_plots.get(col_index) {
                    // Get the tile from the game state
                    let tile = &game_state.board[plot_index];
                    
                    // Determine plot style (highlight special tiles)
                    let border_style = if let Some(harvest_color) = get_harvest_color(&tile.harvest_type) {
                        // Harvest tile - use the harvest type color
                        Style::default().fg(harvest_color)
                    } else if plot_index == 0 || plot_index == 14 {
                        // Special corner tiles - use white instead of yellow
                        Style::default().fg(Color::White)
                    } else if plot_index == 25 || plot_index == 37 {
                        // Keep other special tiles yellow
                        Style::default().fg(Color::Yellow)
                    } else {
                        // Regular tiles
                        Style::default().fg(Color::Gray)
                    };

                    // Get harvest symbol if applicable
                    let harvest_symbol = get_harvest_symbol(&tile.harvest_type);
                    let harvest_color = get_harvest_color(&tile.harvest_type).unwrap_or(Color::White);
                    
                    // Create plot index display with harvest symbol if applicable
                    let index_line = if harvest_symbol.is_empty() {
                        Line::from(vec![
                            Span::styled(format!("{}", plot_index), Style::default().fg(Color::White)),
                        ])
                    } else {
                        Line::from(vec![
                            Span::styled(format!("{}", plot_index), Style::default().fg(Color::White)),
                            // Add a spacer
                            Span::styled(" ", Style::default()),
                            // Add the harvest symbol with appropriate color
                            Span::styled(harvest_symbol, Style::default().fg(harvest_color)),
                        ])
                    };

                    // Create player indicator string
                    let player_indicators = players_by_position
                        .get(&plot_index)
                        .map(|player_ids| {
                            player_ids
                                .iter()
                                .map(|&id| Span::styled("●", Style::default().fg(get_player_color(id))))
                                .collect::<Vec<Span>>()
                        })
                        .unwrap_or_default();

                    // Get a shortened tile name
                    let _short_name = get_short_tile_name(&tile.name);
                    
                    // Create content for the plot cell - include the shortened name if we have space
                    let mut plot_content = vec![
                        index_line.alignment(Alignment::Left), // Align number to top-left
                    ];
                    
                    // Only add player indicators if there are any players on this tile
                    if !player_indicators.is_empty() {
                        plot_content.push(Line::from(player_indicators).alignment(Alignment::Center));
                    } else {
                        // Remove tile names - keep the display simple
                        // Just show a blank line instead
                        plot_content.push(Line::from("").alignment(Alignment::Center));
                    }

                    let plot_paragraph = Paragraph::new(plot_content)
                        .block(Block::default().borders(Borders::ALL).border_style(border_style));

                    frame.render_widget(plot_paragraph, *plot_area);
                }
            } // Else: leave the area blank for alignment
        }
    }
} 