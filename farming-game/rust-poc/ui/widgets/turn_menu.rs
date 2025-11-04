use ratatui::{
    prelude::{Rect, Frame, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Clear},
    text::{Text, Span, Line},
    layout::Alignment,
};
use crate::models::GameState;
use crate::game::GameEffect;

/// Renders the turn menu that appears after a player's turn.
pub fn render_turn_menu(
    frame: &mut Frame,
    area: Rect,
    game_state: &GameState,
    player_id: usize,
    has_otb_cards: bool
) {
    // Create a centered menu box - make it more compact
    let menu_width = 60.min(area.width.saturating_sub(4));
    let menu_height = 12.min(area.height.saturating_sub(4));  // Reduced height
    
    let menu_area = Rect {
        x: (area.width - menu_width) / 2,
        y: (area.height - menu_height) / 2,
        width: menu_width,
        height: menu_height,
    };
    
    // First, render a completely opaque Clear widget to cover text underneath
    frame.render_widget(Clear, menu_area);
    
    // Split the menu into sections - more compact layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),  // Title - reduced from 3
            Constraint::Length(2),  // Player info - reduced from 3
            Constraint::Length(4),  // Options - reduced from 5
            Constraint::Length(1),  // Instructions
        ])
        .split(menu_area);
    
    // Get player information
    let player = &game_state.players[&player_id];
    let player_name = &player.name;
    let player_cash = player.cash;
    let player_debt = player.debt;
    
    // Get available option to buy cards and count affordable ones
    let option_cards = game_state.get_option_to_buy_cards(player_id);
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
    
    // Create title with styling
    let title_text = format!("{}'s Turn Menu", player_name);
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Yellow).bold().bg(Color::Black))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM).bg(Color::Black));
    
    // Player financial info with styling
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
        Span::styled(" | Affordable OTB: ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(format!("{}", affordable_cards), 
            if affordable_cards > 0 { Style::default().fg(Color::Green).bg(Color::Black) }
            else { Style::default().fg(Color::Red).bg(Color::Black) }
        ),
    ];
    
    let player_info = Paragraph::new(Line::from(player_info_text))
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);  // Center align for better appearance
    
    // Create options text with styling
    let mut options_text = Vec::new();
    
    // Show Option to Buy first
    if has_otb_cards {
        options_text.push(Line::from(vec![
            Span::styled("O", Style::default().fg(Color::Cyan).bg(Color::Black).bold()),
            Span::styled(" - View and exercise Option to Buy cards", Style::default().fg(Color::White).bg(Color::Black)),
        ]));
    } else {
        options_text.push(Line::from(vec![
            Span::styled("O", Style::default().fg(Color::DarkGray).bg(Color::Black)),
            Span::styled(" - No Option to Buy cards available", Style::default().fg(Color::DarkGray).bg(Color::Black)),
        ]));
    }
    
    // Add option to pay back loans
    if player_cash > 0 && player_debt > 0 {
        options_text.push(Line::from(vec![
            Span::styled("P", Style::default().fg(Color::Cyan).bg(Color::Black).bold()),
            Span::styled(" - Pay back loans", Style::default().fg(Color::White).bg(Color::Black)),
        ]));
    } else {
        options_text.push(Line::from(vec![
            Span::styled("P", Style::default().fg(Color::DarkGray).bg(Color::Black)),
            Span::styled(" - No cash available to pay loans", Style::default().fg(Color::DarkGray).bg(Color::Black)),
        ]));
    }

    // Add end turn option last
    options_text.push(Line::from(vec![
        Span::styled("E", Style::default().fg(Color::Cyan).bg(Color::Black).bold()),
        Span::styled(" - End turn and move to the next player", Style::default().fg(Color::White).bg(Color::Black)),
    ]));
    
    let options_paragraph = Paragraph::new(Text::from(options_text))
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::NONE).bg(Color::Black));
    
    // Instructions
    let instructions = Paragraph::new("Press the highlighted key to select an option")
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE).bg(Color::Black));
    
    // Render everything
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Turn Options")
            .bg(Color::Black),
        menu_area
    );
    
    frame.render_widget(title, chunks[0]);
    frame.render_widget(player_info, chunks[1]);
    frame.render_widget(options_paragraph, chunks[2]);
    frame.render_widget(instructions, chunks[3]);
} 