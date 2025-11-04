use ratatui::{
    prelude::{Rect, Frame, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState, Clear},
    text::{Span, Line},
};
use crate::models::{GameState, asset::AssetType};
use crate::game::GameEffect;

/// Renders an option to buy dialog for player decisions.
pub fn render_option_dialog(
    frame: &mut Frame, 
    area: Rect, 
    game_state: &GameState, 
    player_id: usize, 
    selected_index: usize
) {
    // Create a centered dialog box - make it wider and much taller
    let dialog_width = 80.min(area.width.saturating_sub(4));
    let dialog_height = 20.min(area.height.saturating_sub(4));  // Adjusted for better proportions
    
    let dialog_area = Rect {
        x: (area.width - dialog_width) / 2,
        y: (area.height - dialog_height) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    // First, render a completely opaque Clear widget to cover any text underneath
    frame.render_widget(Clear, dialog_area);
    
    // Split the dialog into sections using fixed heights
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),    // Title
            Constraint::Length(8),    // Card list - fixed height for ~4 cards
            Constraint::Length(3),    // Player info
            Constraint::Length(3),    // Instructions
        ])
        .split(dialog_area);
    
    // Get player information
    let player = game_state.players.get(&player_id).unwrap();
    let player_name = &player.name;
    let player_cash = player.cash;
    let player_debt = player.debt;
    let player_position = player.position;
    
    // Get available option to buy cards
    let option_cards = game_state.get_option_to_buy_cards(player_id);
    
    // Count affordable O.T.B. cards
    let affordable_cards = option_cards.iter().filter(|card| {
        match &card.effect {
            GameEffect::OptionalBuyAsset { cost, .. } => {
                player.cash >= *cost || game_state._check_option_to_buy_loan(player_id, card.id).is_ok()
            },
            GameEffect::LeaseRidge { cost, .. } => {
                player.cash >= *cost || game_state._check_option_to_buy_loan(player_id, card.id).is_ok()
            },
            _ => false,
        }
    }).count();
    
    // Create title with enhanced styling
    let title_text = format!("{}'s Option to Buy Cards", player_name);
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Yellow).bold().bg(Color::Black))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM).bg(Color::Black));
    
    // Create card list items
    let mut list_items = Vec::new();
    for (i, card) in option_cards.iter().enumerate() {
        // Check if player can afford the down payment using the loan system
        let (card_details, can_afford, loan_needed) = match &card.effect {
            GameEffect::OptionalBuyAsset { asset, quantity, cost } => {
                // Check if player can directly afford it or can get a loan
                let direct_purchase = player.cash >= *cost;
                
                // If not direct purchase, check if they can make the down payment
                let can_get_loan = if !direct_purchase {
                    game_state._check_option_to_buy_loan(player_id, card.id).is_ok()
                } else {
                    false // Don't need a loan
                };
                
                (format!("{} x{} - ${} - {}", 
                    format_asset_type(*asset), 
                    quantity, 
                    cost,
                    card.title
                ), direct_purchase || can_get_loan, !direct_purchase && can_get_loan)
            },
            GameEffect::LeaseRidge { name, cost, cow_count } => {
                // Check if player can directly afford it or can get a loan
                let direct_purchase = player.cash >= *cost;
                
                // If not direct purchase, check if they can make the down payment
                let can_get_loan = if !direct_purchase {
                    game_state._check_option_to_buy_loan(player_id, card.id).is_ok()
                } else {
                    false // Don't need a loan
                };
                
                (format!("Ridge: {} - ${} - {} cows", 
                    name, 
                    cost,
                    cow_count
                ), direct_purchase || can_get_loan, !direct_purchase && can_get_loan)
            },
            _ => ("Unknown card type".to_string(), false, false),
        };
        
        // Check if OTB is disabled due to position
        let is_disabled = player_position >= 15 && player_position <= 48;
        
        // Display affordability status with icons
        let status = if is_disabled {
            " 🔒"  // Locked for positions 15-48
        } else if can_afford {
            if loan_needed {
                " 💰+💳"  // Money + Credit card for loan
            } else {
                " ✅💰"  // Checkmark + Money for cash purchase
            }
        } else {
            " ❌"  // X mark for cannot afford
        };
        
        // Set style based on selected state, affordability, and position
        let style = if i == selected_index {
            Style::default().fg(Color::Black).bg(Color::White)
        } else if is_disabled {
            Style::default().fg(Color::DarkGray).bg(Color::Black)
        } else if !can_afford {
            Style::default().fg(Color::DarkGray).bg(Color::Black)
        } else if loan_needed {
            Style::default().fg(Color::Yellow).bg(Color::Black) // Yellow for loan
        } else {
            Style::default().fg(Color::Green).bg(Color::Black)  // Green for cash purchase
        };
        
        list_items.push(ListItem::new(format!("{}{}", card_details, status)).style(style));
    }
    
    // Fill empty space if there are no cards
    if option_cards.is_empty() {
        list_items.push(ListItem::new("No cards available").style(Style::default().fg(Color::DarkGray).bg(Color::Black)));
    }
    
    // Create list widget with items and background
    let mut list_state = ListState::default().with_selected(Some(selected_index));
    
    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Available Cards").bg(Color::Black))
        .style(Style::default().bg(Color::Black))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))  // Added highlight style
        .highlight_symbol(">> ");  // Added highlight symbol
    
    // Player information with background and enhanced display
    let player_cash_style = if player_cash > 3000 {
        Style::default().fg(Color::Green).bg(Color::Black)
    } else if player_cash > 1000 {
        Style::default().fg(Color::Yellow).bg(Color::Black)
    } else {
        Style::default().fg(Color::Red).bg(Color::Black)
    };
    
    let player_debt_style = if player_debt < 5000 {
        Style::default().fg(Color::Green).bg(Color::Black)
    } else if player_debt < 10000 {
        Style::default().fg(Color::Yellow).bg(Color::Black)
    } else {
        Style::default().fg(Color::Red).bg(Color::Black)
    };
    
    let player_info_text = vec![
        Span::styled("Cash: ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(format!("${} ", player_cash), player_cash_style),
        Span::styled("| Debt: ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(format!("${}", player_debt), player_debt_style),
        Span::styled(" | Affordable O.T.B.: ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(format!("{}", affordable_cards), 
            if affordable_cards > 0 { Style::default().fg(Color::Green).bg(Color::Black) }
            else { Style::default().fg(Color::Red).bg(Color::Black) }
        ),
    ];
    
    let player_info = Paragraph::new(Line::from(player_info_text))
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::ALL).title("Player Finances").bg(Color::Black));
    
    // Instructions with improved styling and icons
    let instructions = if player_position >= 15 && player_position <= 48 {
        "O.T.B. cards are locked in positions 15-48"
    } else {
        "↑/↓: Select card | Enter: Buy | Esc: Skip"
    };
    
    let instructions = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::TOP).bg(Color::Black));
    
    // Render everything in the correct order:
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Option to Buy")
            .bg(Color::Black),
        dialog_area
    );
    
    // Render the components
    frame.render_widget(title, chunks[0]);
    frame.render_stateful_widget(list, chunks[1], &mut list_state);
    frame.render_widget(player_info, chunks[2]);
    frame.render_widget(instructions, chunks[3]);
}

/// Helper function to format asset type names for display
fn format_asset_type(asset_type: AssetType) -> String {
    match asset_type {
        AssetType::Grain => "Grain".to_string(),
        AssetType::Hay => "Hay".to_string(),
        AssetType::Cows => "Cattle".to_string(),
        AssetType::Fruit => "Fruit".to_string(), 
        AssetType::Tractor => "Tractor".to_string(),
        AssetType::Harvester => "Harvester".to_string(),
    }
} 