// src/ui/app.rs

use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect, Frame, Margin, Style, Color},
    widgets::{Paragraph},
};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::ui::terminal::Tui;
use crate::ui::widgets::scoreboard::render_scoreboard;
use crate::ui::widgets::log::render_log;
use crate::ui::widgets::option_dialog::render_option_dialog;
use crate::ui::widgets::turn_menu::render_turn_menu;
use crate::ui::widgets::loan_payment::render_loan_payment;
use crate::models::GameState;
use crate::game::GameEffect;
use crate::config::WINNING_NET_WORTH;

/// Helper function to create a centered rect with fixed dimensions, inset by 1 cell.
fn centered_fixed_rect(width: u16, height: u16, r: Rect) -> Rect {
    // Calculate available inner dimensions assuming a 1-cell border on the parent
    let inner_height = r.height.saturating_sub(2);
    let inner_width = r.width.saturating_sub(2);

    // Ensure requested dimensions are not larger than available inner space
    let popup_height = height.min(inner_height);
    let popup_width = width.min(inner_width);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((inner_height.saturating_sub(popup_height)) / 2), // Top margin within inner area
            Constraint::Length(popup_height),
            Constraint::Min(0),
        ])
        .split(r.inner(&Margin { vertical: 1, horizontal: 1 })); // Use Margin directly

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((inner_width.saturating_sub(popup_width)) / 2), // Left margin within inner area
            Constraint::Length(popup_width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}

/// Game UI states
enum UiState {
    /// Normal gameplay
    Game,
    /// Turn menu after player has moved
    TurnMenu {
        player_id: usize,
    },
    /// Option to Buy decision
    OptionToBuy {
        player_id: usize,
        selected_index: usize,
    },
    /// Loan payment screen
    LoanPayment {
        player_id: usize,
        payment_amount: i32,
    },
}

/// Represents the main application state.
pub struct App {
    running: bool, // Flag to control the main loop
    game_state: GameState, // Add GameState to App
    log_entries: Vec<String>, // Add log storage
    log_scroll_offset: usize, // Track log scroll position
    ui_state: UiState, // Current UI state
    rng: StdRng, // Add dedicated RNG
}

impl App {
    /// Creates a new App instance from a pre-initialized GameState.
    pub fn new(game_state: GameState) -> Self { // Accept GameState
        let mut app = Self {
            running: true,
            game_state: game_state.clone(), // Clone to access first player info
            log_entries: Vec::new(), // Initialize empty logs
            log_scroll_offset: 0,
            ui_state: UiState::Game,
            rng: StdRng::from_entropy(), // Initialize RNG from entropy
        };

        // Add initial logs without the scrolling instructions
        app.add_log_entry("Game initialized.".to_string());
        app.add_log_entry("Scoreboard TUI setup complete.".to_string());
        app.add_log_entry("".to_string()); // Add blank line after instructions

        // Add first player's turn message
        let first_player = &app.game_state.players[&app.game_state.turn_order[0]].name;
        app.add_log_entry(format!("--- {}'s turn (Press Enter to roll) ---", first_player));
        
        app
    }

    /// Helper function to capitalize the first letter of a message
    fn capitalize_first_letter(message: String) -> String {
        let mut chars = message.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    /// Adds a message to the log and attempts to scroll to the bottom.
    fn add_log_entry(&mut self, message: String) {
        // Store the current scroll position to check if we're already scrolled to bottom
        let previous_max = if self.log_entries.len() > 0 {
            // Conservative estimate of visible lines in log area
            let estimated_visible_lines = 20;
            self.log_entries.len().saturating_sub(estimated_visible_lines)
        } else {
            0
        };
        
        // Check if we were already at the bottom (or special MAX value)
        let was_at_bottom = self.log_scroll_offset == usize::MAX || 
                            self.log_scroll_offset >= previous_max;
        
        // Capitalize the first letter unless it's a turn header or emoji
        let message = if message.starts_with("---") || message.starts_with("🎲") || 
                        message.starts_with("💰") || message.starts_with("💸") || 
                        message.starts_with("🃏") || message.starts_with("❌") || 
                        message.starts_with("🌾") || message.starts_with("⏭️") || 
                        message.starts_with("ℹ️") {
            message
        } else {
            Self::capitalize_first_letter(message)
        };
        
        // Add the message
        self.log_entries.push(message);
        
        // Only auto-scroll if we were already at the bottom
        if was_at_bottom {
            self.scroll_log_to_bottom();
        }
        // Otherwise, maintain current scroll position
    }

    /// Runs the main application loop.
    pub fn run(&mut self, tui: &mut Tui) -> io::Result<()> {
        while self.running {
            // 1. Draw the UI
            tui.draw(|frame| {
                self.ui(frame);
            })?;

            // 2. Handle events
            if event::poll(Duration::from_millis(50))? { // Poll for events with a timeout
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        // Handle scrolling in all UI states with dedicated keys
                        match key.code {
                            _ => {
                                // Regular state-specific key handling with shift modifiers for scroll
                                if key.modifiers.contains(event::KeyModifiers::SHIFT) {
                                    match key.code {
                                        KeyCode::Up => self.scroll_log_up(),
                                        KeyCode::Down => self.scroll_log_down(),
                                        KeyCode::PageUp => self.scroll_log_page_up(),
                                        KeyCode::PageDown => self.scroll_log_page_down(),
                                        KeyCode::Home => self.scroll_log_to_top(),
                                        KeyCode::End => self.scroll_log_to_bottom(),
                                        _ => {}
                                    }
                                } else {
                                    // Regular state-specific key handling
                                    match &mut self.ui_state {
                                        UiState::Game => match key.code {
                                            KeyCode::Char('q') => self.quit(), // Quit on 'q'
                                            KeyCode::Enter => self.advance_turn(),
                                            _ => {} // Handle other keys later
                                        },
                                        UiState::TurnMenu { player_id } => {
                                            let current_player_id = *player_id;
                                            match key.code {
                                                KeyCode::Char('q') => self.quit(),
                                                KeyCode::Char('e') | KeyCode::Char('E') => {
                                                    // End turn and move to next player
                                                    self.end_turn();
                                                },
                                                KeyCode::Char('o') | KeyCode::Char('O') => {
                                                    // Check if player has O.T.B. cards
                                                    let option_cards = self.game_state.get_option_to_buy_cards(current_player_id);
                                                    if !option_cards.is_empty() && self.game_state.can_exercise_option_to_buy(current_player_id) {
                                                        // Show O.T.B. dialog
                                                        self.ui_state = UiState::OptionToBuy {
                                                            player_id: current_player_id,
                                                            selected_index: 0,
                                                        };
                                                    } else {
                                                        self.add_log_entry("O.T.B. unavailable at this time of the year.".to_string());
                                                    }
                                                },
                                                KeyCode::Char('p') | KeyCode::Char('P') => {
                                                    // Only show loan payment dialog if player has cash and debt
                                                    let player = &self.game_state.players[&current_player_id];
                                                    if player.cash > 0 && player.debt > 0 {
                                                        // Show loan payment dialog - start with 10% of debt or cash (whichever is less)
                                                        let default_payment = (player.debt / 10).min(player.cash);
                                                        self.ui_state = UiState::LoanPayment {
                                                            player_id: current_player_id,
                                                            payment_amount: default_payment,
                                                        };
                                                    } else {
                                                        self.add_log_entry("Cannot pay loans - no cash available.".to_string());
                                                    }
                                                },
                                                _ => {}
                                            }
                                        },
                                        UiState::OptionToBuy { player_id, selected_index } => match key.code {
                                            KeyCode::Char('q') => self.quit(),
                                            KeyCode::Char('e') => {
                                                // Return to turn menu
                                                self.ui_state = UiState::TurnMenu {
                                                    player_id: *player_id
                                                };
                                            },
                                            KeyCode::Esc => {
                                                // Return to turn menu
                                                self.ui_state = UiState::TurnMenu {
                                                    player_id: *player_id
                                                };
                                            },
                                            KeyCode::Up => {
                                                // Move selection up
                                                let cards = self.game_state.get_option_to_buy_cards(*player_id);
                                                if !cards.is_empty() && *selected_index > 0 {
                                                    *selected_index -= 1;
                                                }
                                            },
                                            KeyCode::Down => {
                                                // Move selection down
                                                let cards = self.game_state.get_option_to_buy_cards(*player_id);
                                                if !cards.is_empty() && *selected_index < cards.len() - 1 {
                                                    *selected_index += 1;
                                                }
                                            },
                                            KeyCode::Enter => {
                                                // Process the option to buy
                                                let player_id = *player_id;
                                                let selected_idx = *selected_index;
                                                self.process_option_to_buy(player_id, selected_idx);
                                            },
                                            _ => {}
                                        },
                                        UiState::LoanPayment { player_id, payment_amount } => match key.code {
                                            KeyCode::Char('q') => self.quit(),
                                            KeyCode::Char('e') => {
                                                // Return to turn menu
                                                self.ui_state = UiState::TurnMenu {
                                                    player_id: *player_id
                                                };
                                            },
                                            KeyCode::Esc => {
                                                // Return to turn menu
                                                self.ui_state = UiState::TurnMenu {
                                                    player_id: *player_id
                                                };
                                            },
                                            KeyCode::Up => {
                                                // Increase payment - step by 100
                                                let player = &self.game_state.players[player_id];
                                                *payment_amount = (*payment_amount + 100).min(player.cash.min(player.debt));
                                            },
                                            KeyCode::Down => {
                                                // Decrease payment - step by 100, minimum 0
                                                *payment_amount = (*payment_amount - 100).max(0);
                                            },
                                            KeyCode::PageUp => {
                                                // Increase payment - step by 1000
                                                let player = &self.game_state.players[player_id];
                                                *payment_amount = (*payment_amount + 1000).min(player.cash.min(player.debt));
                                            },
                                            KeyCode::PageDown => {
                                                // Decrease payment - step by 1000, minimum 0
                                                *payment_amount = (*payment_amount - 1000).max(0);
                                            },
                                            KeyCode::Enter => {
                                                // Process loan payment
                                                let player_id = *player_id;
                                                let payment = *payment_amount;
                                                self.pay_loan(player_id, payment);
                                                
                                                // Return to turn menu
                                                self.ui_state = UiState::TurnMenu {
                                                    player_id
                                                };
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Add other event handling (mouse, resize) later if needed
        }
        Ok(())
    }
    
    /// Process loan payment
    fn pay_loan(&mut self, player_id: usize, payment_amount: i32) {
        if payment_amount <= 0 {
            self.add_log_entry("No payment made.".to_string());
            return;
        }
        
        // Check conditions before borrowing
        {
            let player = &self.game_state.players[&player_id];
            
            // Check if player has enough cash
            if player.cash < payment_amount {
                self.add_log_entry(format!("Error: Not enough cash for payment of ${}", payment_amount));
                return;
            }
            
            // Check if player has debt to pay
            if player.debt <= 0 {
                self.add_log_entry("Error: No debt to pay.".to_string());
                return;
            }
        }
        
        // Now apply the payment with a separate borrow
        {
            let player = self.game_state.players.get_mut(&player_id).unwrap();
            let player_name = player.name.clone();
            
            // Apply the payment
            player.cash -= payment_amount;
            player.debt -= payment_amount;
            
            // Update net worth in the same borrow
            player.net_worth = player.cash - player.debt + player.total_asset_value + player.total_ridge_value;
            
            // Read remaining debt *before* calling add_log_entry to avoid double mutable borrow
            let remaining_debt = player.debt;

            self.add_log_entry(format!("{} paid ${} towards debt. Remaining debt: ${}",
                player_name, payment_amount, remaining_debt)); // Use the local variable
        }
    }
    
    /// Process an option to buy selection
    fn process_option_to_buy(&mut self, player_id: usize, selected_index: usize) {
        // Get all needed data before making mutable calls
        let cards = self.game_state.get_option_to_buy_cards(player_id);
        if cards.is_empty() || selected_index >= cards.len() {
            self.add_log_entry("Error: Invalid card selection.".to_string());
            
            // Return to turn menu
            self.ui_state = UiState::TurnMenu {
                player_id
            };
            return;
        }
        
        // Clone the card data we need
        let card = cards[selected_index];
        let card_id = card.id;
        let card_title = card.title.clone();
        
        // Get player name before the mutable borrow
        let player_name = self.game_state.players[&player_id].name.clone();
        
        // First, check if the player has enough cash for direct purchase
        let needs_loan = {
            let player = &self.game_state.players[&player_id];
            let cost = match &card.effect {
                GameEffect::OptionalBuyAsset { cost, .. } => *cost,
                GameEffect::LeaseRidge { cost, .. } => *cost,
                _ => 0,
            };
            player.cash < cost
        };
        
        // Always try with direct purchase first (confirm_loan = false)
        let purchase_result = self.game_state.exercise_option_to_buy(player_id, card_id, false);
        
        // If direct purchase fails due to needing a loan, try with loan
        match purchase_result {
            Ok(_) => {
                self.add_log_entry(format!("{} exercised O.T.B.: {}", 
                    player_name, card_title));
                
                // Return to turn menu
                self.ui_state = UiState::TurnMenu {
                    player_id
                };
            },
            Err(e) => {
                // If the error is about loan confirmation and we know the player needs a loan
                if e == "Loan confirmation required" && needs_loan {
                    // Try again with loan confirmation
                    match self.game_state.exercise_option_to_buy(player_id, card_id, true) {
                        Ok(_) => {
                            self.add_log_entry(format!("{} exercised O.T.B.: {} (with loan)", 
                                player_name, card_title));
                            
                            // Return to turn menu
                            self.ui_state = UiState::TurnMenu {
                                player_id
                            };
                        },
                        Err(e) => {
                            // Log the error but stay in O.T.B. dialog
                            self.add_log_entry(format!("Could not exercise option: {}", e));
                        }
                    }
                } else if e.contains("Insufficient funds") {
                    // Log the error but stay in O.T.B. dialog
                    self.add_log_entry(format!("Could not exercise option: {}", e));
                } else {
                    self.add_log_entry(format!("Could not exercise option: {}", e));
                    
                    // Return to turn menu
                    self.ui_state = UiState::TurnMenu {
                        player_id
                    };
                }
            }
        }
    }

    /// Ends the current player's turn and advances to the next player
    fn end_turn(&mut self) {
        // Get current player and check for win condition
        let current_player_id = self.game_state.turn_order[self.game_state.current_turn_index];
        
        // Extract needed values before borrowing self as mutable
        let player_name = self.game_state.players[&current_player_id].name.clone();
        let player_net_worth = self.game_state.players[&current_player_id].net_worth;
        
        // Check if current player has won
        if player_net_worth >= WINNING_NET_WORTH {
            // Player has won!
            self.add_log_entry(format!("🏆 {} HAS WON THE GAME! 🏆", player_name));
            self.add_log_entry(format!("Net worth of ${} exceeds the ${} needed to win!", 
                                      player_net_worth, WINNING_NET_WORTH));
            
            // Continue the game but make it clear they've won
            self.add_log_entry("The game can continue, but victory has been achieved.".to_string());
        }
        
        // Advance to the next player's turn
        self.game_state.current_turn_index = 
            (self.game_state.current_turn_index + 1) % self.game_state.turn_order.len();
        
        // Add message for the next player's turn
        let next_player = &self.game_state.players[&self.game_state.turn_order[self.game_state.current_turn_index]].name;
        self.add_log_entry(format!("--- {}'s turn (Press Enter to roll) ---", next_player));
        
        // Return to normal gameplay state
        self.ui_state = UiState::Game;
    }

    /// Scrolls the log up by one line.
    fn scroll_log_up(&mut self) {
        if self.log_scroll_offset > 0 {
            self.log_scroll_offset = self.log_scroll_offset.saturating_sub(1);
        }
    }

    /// Scrolls the log down by one line.
    fn scroll_log_down(&mut self) {
        // Calculate max scroll offset based on content length and visible area
        // We need an estimate since we can't access the render frame here
        let estimated_visible_lines = 20; // Conservative estimate of visible lines in log area
        let max_scroll = self.log_entries.len().saturating_sub(estimated_visible_lines);
        
        // Only increment if we're not already at the max
        if self.log_scroll_offset < max_scroll {
            self.log_scroll_offset = self.log_scroll_offset.saturating_add(1);
        }
    }

    /// Scrolls the log up by a page (e.g., 10 lines).
    fn scroll_log_page_up(&mut self) {
        // Assuming page size of 10 for now
        self.log_scroll_offset = self.log_scroll_offset.saturating_sub(10);
    }

    /// Scrolls the log down by a page (e.g., 10 lines).
    fn scroll_log_page_down(&mut self) {
        // Calculate max scroll as in scroll_log_down
        let estimated_visible_lines = 20;
        let max_scroll = self.log_entries.len().saturating_sub(estimated_visible_lines);
        
        // Add a page but don't exceed max
        self.log_scroll_offset = (self.log_scroll_offset + 10).min(max_scroll);
    }

    /// Scrolls to the top of the log.
    fn scroll_log_to_top(&mut self) {
        self.log_scroll_offset = 0;
    }

    /// Scrolls to the bottom of the log.
    fn scroll_log_to_bottom(&mut self) {
        // Set offset to MAX to signal scrolling to the bottom.
        // The render_log function will clamp this to the actual maximum.
        self.log_scroll_offset = usize::MAX;
    }

    /// Check if a player can perform any meaningful actions (pay debt or use O.T.B. cards)
    fn can_player_perform_actions(&self, player_id: usize) -> bool {
        let player = &self.game_state.players[&player_id];
        
        // Check if player has any cash to pay debt
        let can_pay_debt = player.cash > 0 && player.debt > 0;
        
        // Check if player has O.T.B. cards and can afford them
        let option_cards = self.game_state.get_option_to_buy_cards(player_id);
        let can_use_otb = !option_cards.is_empty() && 
                          self.game_state.can_exercise_option_to_buy(player_id) &&
                          option_cards.iter().any(|card| {
                              // Calculate card cost
                              let cost = match &card.effect {
                                  GameEffect::OptionalBuyAsset { cost, .. } => *cost,
                                  GameEffect::LeaseRidge { cost, .. } => *cost,
                                  _ => 0,
                              };
                              
                              // Check if player can directly afford it or via loan
                              player.cash >= cost || 
                              self.game_state._check_option_to_buy_loan(player_id, card.id).is_ok()
                          });
        
        // Return true if player can perform any action
        can_pay_debt || can_use_otb
    }

    /// Advances the game state by one turn.
    fn advance_turn(&mut self) {
        // Get current player info
        let current_player_id = self.game_state.turn_order[self.game_state.current_turn_index];
        let player_name = self.game_state.players[&current_player_id].name.clone();

        // Simulate a dice roll (1-6) using the App's RNG
        let roll = self.rng.gen_range(1..=6);

        // Clean old logs if they get too large (keeps memory usage in check)
        if self.log_entries.len() > 1000 {
            self.log_entries.drain(0..500);
        }

        // Call the actual game logic
        match crate::game::game_loop::handle_player_turn(
            &mut self.game_state,
            current_player_id,
            roll,
        ) {
            Ok(turn_logs) => {
                // Add all logs returned from the successful turn
                for log_msg in turn_logs {
                    // Skip standalone "landed on" messages, but keep roll messages
                    if !log_msg.contains("Landed on") || log_msg.contains("🎲") {
                        // Remove player name from messages since it's in the header
                        let msg = log_msg.replace(&format!("{} ", player_name), "");
                        self.add_log_entry(msg);
                    }
                }
            }
            Err(e) => {
                // Handle any errors from the game logic
                self.add_log_entry(format!("Error during turn: {}", e));
            }
        }

        // Check if player can perform any meaningful actions
        if !self.can_player_perform_actions(current_player_id) {
            self.add_log_entry("No affordable actions - advancing to next player.".to_string());
            self.end_turn();
            return;
        }

        // After handling movement and effects, show the turn menu
        self.ui_state = UiState::TurnMenu {
            player_id: current_player_id,
        };
    }

    /// Sets the running flag to false to exit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    /// Renders the user interface widgets.
    fn ui(&self, frame: &mut Frame) {
        // Define the main layout: Scoreboard top, Game Board/Log below, Status bar bottom
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(9),    // Scoreboard fixed height (title + 6 players + borders)
                Constraint::Min(0),       // Game Board/Log take remaining space
                Constraint::Length(1),    // Status bar
            ])
            .split(frame.size());

        let scoreboard_area = main_layout[0];
        let bottom_area = main_layout[1];
        let status_bar_area = main_layout[2];

        // Define the bottom layout: Game Board left (50%), Log right (50%)
        let bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(bottom_area);

        let game_board_area = bottom_layout[0];
        let log_area = bottom_layout[1];

        // Render main widgets
        render_scoreboard(frame, scoreboard_area, &self.game_state);
        crate::ui::widgets::game_board::render_game_board(frame, game_board_area, &self.game_state);
        render_log(frame, log_area, &self.log_entries, self.log_scroll_offset);

        // Render status bar with key instructions
        let status_text = match self.ui_state {
            UiState::Game => "q: Quit | Enter: Roll | Shift+↑/↓: Scroll | Shift+PgUp/PgDn: Page | Shift+Home/End: Top/Bottom",
            UiState::TurnMenu { .. } => "O: Option to Buy | P: Pay Loan | E: End Turn | Esc: Skip | Shift+↑/↓: Scroll | Shift+PgUp/PgDn: Page",
            UiState::OptionToBuy { .. } => "↑/↓: Select card | Enter: Buy | Esc: Skip | Shift+↑/↓: Scroll | Shift+PgUp/PgDn: Page",
            UiState::LoanPayment { .. } => "↑/↓: Adjust by $100 | PgUp/PgDn: Adjust by $1000 | Enter: Confirm | Esc: Cancel | Shift+↑/↓: Scroll",
        };
        
        let status_bar = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(status_bar, status_bar_area);

        // Conditionally render dialogs/menus on top, centered within game_board_area
        match &self.ui_state {
            UiState::TurnMenu { player_id } => {
                let has_otb_cards = !self.game_state.get_option_to_buy_cards(*player_id).is_empty() && 
                                    self.game_state.can_exercise_option_to_buy(*player_id);
                
                // Calculate centered rect for turn menu (e.g., 60x15)
                let popup_area = centered_fixed_rect(60, 15, game_board_area);
                render_turn_menu(frame, popup_area, &self.game_state, *player_id, has_otb_cards);
            },
            UiState::OptionToBuy { player_id, selected_index } => {
                // Calculate centered rect for O.T.B. dialog (reduced height: 80x20)
                let popup_area = centered_fixed_rect(80, 20, game_board_area);
                render_option_dialog(frame, popup_area, &self.game_state, *player_id, *selected_index);
            },
            UiState::LoanPayment { player_id, payment_amount } => {
                // Calculate centered rect for loan payment (e.g., 60x10)
                let popup_area = centered_fixed_rect(60, 10, game_board_area);
                let mut payment = *payment_amount;
                render_loan_payment(frame, popup_area, &self.game_state, *player_id, &mut payment);
            },
            _ => {}
        }
    }
} 