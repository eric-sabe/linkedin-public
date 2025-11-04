use ratatui::{
    prelude::{Rect, Frame, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Clear},
    text::{Text, Span, Line},
    layout::Alignment,
};
use crate::models::GameState;

/// Renders a loan payment dialog for player to pay down debt.
pub fn render_loan_payment(
    frame: &mut Frame,
    area: Rect,
    game_state: &GameState,
    player_id: usize,
    payment_amount: &mut i32
) {
    // Create a centered dialog box
    let dialog_width = 60.min(area.width.saturating_sub(4));
    let dialog_height = 16.min(area.height.saturating_sub(4));
    
    let dialog_area = Rect {
        x: (area.width - dialog_width) / 2,
        y: (area.height - dialog_height) / 2,
        width: dialog_width,
        height: dialog_height,
    };
    
    // First, render a completely opaque Clear widget to cover text underneath
    frame.render_widget(Clear, dialog_area);
    
    // Split the dialog into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Player info
            Constraint::Length(5),  // Payment amount and controls
            Constraint::Length(3),  // Action buttons
        ])
        .split(dialog_area);
    
    // Get player information
    let player = &game_state.players[&player_id];
    let player_name = &player.name;
    let player_cash = player.cash;
    let player_debt = player.debt;
    
    // Create title with styling
    let title_text = format!("{}'s Loan Payment", player_name);
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Yellow).bold().bg(Color::Black))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM).bg(Color::Black));
    
    // Player financial info
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
        Line::from(vec![
            Span::styled("Available Cash: ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(format!("${}", player_cash), player_cash_style),
        ]),
        Line::from(vec![
            Span::styled("Current Debt: ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(format!("${}", player_debt), player_debt_style),
        ]),
    ];
    
    let player_info = Paragraph::new(Text::from(player_info_text))
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::NONE).bg(Color::Black));
    
    // Payment amount and controls
    // Ensure payment amount is valid
    *payment_amount = (*payment_amount).clamp(0, player_cash.min(player_debt));
    
    let remaining_cash = player_cash - *payment_amount;
    let remaining_debt = player_debt - *payment_amount;
    
    // Create incrementer display with +/- buttons
    let payment_text = vec![
        Line::from(vec![
            Span::styled("Payment Amount: ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(" $", Style::default().fg(Color::Yellow).bg(Color::Black)),
            Span::styled(format!("{}", payment_amount), Style::default().fg(Color::Yellow).bg(Color::Black).bold()),
            Span::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled("(↑/↓: ±$100)", Style::default().fg(Color::DarkGray).bg(Color::Black)),
            Span::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled("(PgUp/PgDn: ±$1000)", Style::default().fg(Color::DarkGray).bg(Color::Black)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Remaining Cash: ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(format!("${}", remaining_cash), Style::default().fg(Color::Cyan).bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled("Remaining Debt: ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(format!("${}", remaining_debt), Style::default().fg(Color::Cyan).bg(Color::Black)),
        ]),
    ];
    
    let payment_info = Paragraph::new(Text::from(payment_text))
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::NONE).bg(Color::Black));
    
    // Action buttons
    let action_buttons = vec![
        Line::from(vec![
            Span::styled("  ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(" CONFIRM ", Style::default().fg(Color::Black).bg(Color::Green).bold()),
            Span::styled("  ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(" CANCEL ", Style::default().fg(Color::Black).bg(Color::Red).bold()),
            Span::styled("  ", Style::default().fg(Color::White).bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled(" (ENTER) ", Style::default().fg(Color::White).bg(Color::DarkGray)),
            Span::styled("  ", Style::default().fg(Color::White).bg(Color::Black)),
            Span::styled("  (ESC)  ", Style::default().fg(Color::White).bg(Color::DarkGray)),
            Span::styled("  ", Style::default().fg(Color::White).bg(Color::Black)),
        ]),
    ];
    
    let action_buttons_widget = Paragraph::new(Text::from(action_buttons))
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).bg(Color::Black));
    
    // Render everything
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Pay Back Loan")
            .bg(Color::Black),
        dialog_area
    );
    
    frame.render_widget(title, chunks[0]);
    frame.render_widget(player_info, chunks[1]);
    frame.render_widget(payment_info, chunks[2]);
    frame.render_widget(action_buttons_widget, chunks[3]);
} 