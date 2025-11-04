// src/ui/widgets/log.rs

use ratatui::{
    prelude::{Rect, Frame, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap, ScrollbarState, Scrollbar},
    text::{Text, Span, Line},
};
use std::collections::HashSet;

/// Formats log messages for better readability.
fn format_log_entries(log_entries: &[String]) -> Text {
    let mut formatted_text = Text::default();
    let mut lines: Vec<Line> = Vec::new();
    
    // Track when we're starting a new turn to add extra space
    let mut is_turn_start = false;
    let mut processed_indices: HashSet<usize> = HashSet::new();
    
    for (i, entry) in log_entries.iter().enumerate() {
        // Skip if already processed in a combined message
        if processed_indices.contains(&i) {
            continue;
        }

        // Skip standalone "landed on" messages entirely
        if (entry.contains("landed on") || entry.contains("Moved to position")) && !entry.to_lowercase().contains("rolled a") {
            continue;
        }
        
        // Add extra blank line before turn headers for better separation
        if entry.starts_with("--- ") && entry.ends_with(" ---") {
            // Add separator for turns
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "══════════════════════════════════════════════════",
                Style::default().fg(Color::DarkGray)
            )));
            lines.push(Line::from(""));
            
            // Add the turn header
            lines.push(Line::from(Span::styled(
                entry, 
                Style::default().fg(Color::Cyan).bold()
            )));
            
            is_turn_start = true;
            continue;
        }
        
        // Add blank line after the turn header to separate from actions
        if is_turn_start && !entry.starts_with("---") {
            lines.push(Line::from("")); // Add blank line after header
            is_turn_start = false;
        }
        
        // Skip "Description:" prefix and just show the content
        if entry.starts_with("Description:") {
            let description = entry.trim_start_matches("Description:").trim();
            lines.push(Line::from(vec![
                Span::styled("ℹ️ ", Style::default().fg(Color::Blue).bold()),
                Span::styled(description, Style::default().fg(Color::Blue))
            ]));
            continue;
        }
        
        // Format brief descriptions
        if entry.starts_with("Brief:") {
            let brief = entry.trim_start_matches("Brief:").trim();
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(Color::Blue).bold()),
                Span::styled(brief, Style::default().fg(Color::Blue))
            ]));
            continue;
        }
        
        // Check for warm-related messages first
        if entry.to_lowercase().contains("warm") {
            lines.push(Line::from(vec![
                Span::styled("🌞", Style::default().fg(Color::Yellow).bold()),
                Span::styled(" ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
            continue;
        }
        
        // Format based on message type
        if entry.to_lowercase().contains("error") {
            // Highlight errors in red with icon
            lines.push(Line::from(vec![
                Span::styled("❌ ", Style::default().fg(Color::Red).bold()),
                Span::styled(entry, Style::default().fg(Color::Red))
            ]));
        } else if entry.to_lowercase().contains("drew") {
            // Card draws - first line
            lines.push(Line::from(vec![
                Span::styled("🃏 ", Style::default().fg(Color::Magenta).bold()),
                Span::styled(entry, Style::default().fg(Color::Magenta))
            ]));
            
            // Check if next line is the card description
            if let Some(next_entry) = log_entries.get(i + 1) {
                if next_entry.contains(" - ") {
                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default().fg(Color::Magenta).bold()),
                        Span::styled(next_entry, Style::default().fg(Color::Magenta))
                    ]));
                    processed_indices.insert(i + 1);
                }
            }
        } else if entry.to_lowercase().contains("gained") || 
                  entry.to_lowercase().contains("collected") || 
                  entry.to_lowercase().contains("collect") {
            // Highlight gains in green with money icon
            lines.push(Line::from(vec![
                Span::styled("💰 ", Style::default().fg(Color::Green).bold()),
                Span::styled(entry, Style::default().fg(Color::Green))
            ]));
        } else if entry.to_lowercase().contains("must pay") || 
                  entry.to_lowercase().contains("pay $") || 
                  entry.to_lowercase().contains("paid") || 
                  entry.to_lowercase().contains("debt") {
            // Highlight expenses in yellow with expense icon
            lines.push(Line::from(vec![
                Span::styled("💸 ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
        } else if entry.to_lowercase().contains("interest") {
            // Interest payments/bank related
            lines.push(Line::from(vec![
                Span::styled("🏦 ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
        } else if entry.to_lowercase().contains("rolled a") && !processed_indices.contains(&i) {
            // Dice rolls - combine with landing message if present
            let mut roll_message = entry.to_string();
            if let Some(next_entry) = log_entries.get(i + 1) {
                if next_entry.to_lowercase().contains("landed on") {
                    roll_message = format!("{} - {}", roll_message, next_entry.trim());
                    processed_indices.insert(i + 1);
                }
            }
            lines.push(Line::from(vec![
                Span::styled("🎲 ", Style::default().fg(Color::White).bold()),
                Span::styled(roll_message, Style::default().fg(Color::White))
            ]));
        } else if entry.to_lowercase().contains("stuck") && entry.to_lowercase().contains("mud") {
            // Stuck in mud events
            lines.push(Line::from(vec![
                Span::styled("🚜 ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
        } else if entry.to_lowercase().contains("does not have") || 
                  entry.to_lowercase().contains("don't have") || 
                  entry.to_lowercase().contains("dont have") {
            // Missing asset messages
            lines.push(Line::from(vec![
                Span::styled("❌ ", Style::default().fg(Color::Red).bold()),
                Span::styled(entry, Style::default().fg(Color::Red))
            ]));
        } else if entry.to_lowercase().contains("double yield") || 
                  entry.to_lowercase().contains("yield is doubled") {
            // Double yield messages
            lines.push(Line::from(vec![
                Span::styled("✨ ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
        } else if entry.to_lowercase().contains("exercised o.t.b.") {
            // O.T.B. exercise messages
            lines.push(Line::from(vec![
                Span::styled("🛍️ ", Style::default().fg(Color::Green).bold()),
                Span::styled(entry, Style::default().fg(Color::Green))
            ]));
        } else if entry.to_lowercase().contains("o.t.b. unavailable") {
            // O.T.B. unavailable message
            lines.push(Line::from(vec![
                Span::styled("🔒 ", Style::default().fg(Color::DarkGray).bold()),
                Span::styled(entry, Style::default().fg(Color::DarkGray))
            ]));
        } else if entry.to_lowercase().contains("harvest") {
            // Format harvest messages with a special icon
            lines.push(Line::from(vec![
                Span::styled("🌾 ", Style::default().fg(Color::Green).bold()),
                Span::styled(entry, Style::default().fg(Color::Green))
            ]));
        } else if entry.to_lowercase().contains("mt. st. helens") || 
                  entry.to_lowercase().contains("volcano") {
            // Volcano/Mt. St. Helens events
            lines.push(Line::from(vec![
                Span::styled("🌋 ", Style::default().fg(Color::Red).bold()),
                Span::styled(entry, Style::default().fg(Color::Red))
            ]));
        } else if entry.to_lowercase().contains("irs") || 
                  entry.to_lowercase().contains("garnish") || 
                  entry.to_lowercase().contains("tax") {
            // Government/IRS related
            lines.push(Line::from(vec![
                Span::styled("🏛️ ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
        } else if entry.to_lowercase().contains("hibernate") || 
                  entry.to_lowercase().contains("sleep") {
            // Hibernation/sleep related
            lines.push(Line::from(vec![
                Span::styled("😴 ", Style::default().fg(Color::Blue).bold()),
                Span::styled(entry, Style::default().fg(Color::Blue))
            ]));
        } else if entry.to_lowercase().contains("early") || 
                  entry.to_lowercase().contains("ahead") || 
                  entry.to_lowercase().contains("time") {
            // Time-related events
            lines.push(Line::from(vec![
                Span::styled("⏰ ", Style::default().fg(Color::Cyan).bold()),
                Span::styled(entry, Style::default().fg(Color::Cyan))
            ]));
        } else if (entry.to_lowercase().contains("skip") && entry.to_lowercase().contains("year")) || 
                  (entry.to_lowercase().contains("hurt") && entry.to_lowercase().contains("back")) {
            // Skip year effect
            lines.push(Line::from(vec![
                Span::styled("⏭️ ", Style::default().fg(Color::Red).bold()),
                Span::styled(entry, Style::default().fg(Color::Red))
            ]));
        } else if entry.to_lowercase().contains("rainy day") {
            // Rainy day messages
            lines.push(Line::from(vec![
                Span::styled("🌧️ ", Style::default().fg(Color::Blue).bold()),
                Span::styled(entry, Style::default().fg(Color::Blue))
            ]));
        } else if entry.to_lowercase().contains("no affordable actions") {
            // No affordable actions message
            lines.push(Line::from(vec![
                Span::styled("➡️ ", Style::default().fg(Color::Blue).bold()),
                Span::styled(entry, Style::default().fg(Color::Blue))
            ]));
        } else if entry.to_lowercase().contains("no income for you") {
            // No income message
            lines.push(Line::from(vec![
                Span::styled("🚫 ", Style::default().fg(Color::Red).bold()),
                Span::styled(entry, Style::default().fg(Color::Red))
            ]));
        } else if entry.to_lowercase().contains("moved to") && !entry.to_lowercase().contains("no affordable actions") {
            // Movement messages (but not "No affordable actions" messages)
            lines.push(Line::from(vec![
                Span::styled("➡️ ", Style::default().fg(Color::Blue).bold()),
                Span::styled(entry, Style::default().fg(Color::Blue))
            ]));
        } else if entry.to_lowercase().contains("operating expense:") {
            // Operating expense messages
            lines.push(Line::from(vec![
                Span::styled("💼 ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(entry, Style::default().fg(Color::Yellow))
            ]));
        } else if entry.to_lowercase().contains("hay:") || 
                  entry.to_lowercase().contains("wheat:") ||
                  entry.to_lowercase().contains("corn:") ||
                  entry.to_lowercase().contains("apple:") ||
                  entry.to_lowercase().contains("cherry:") {
            // Crop harvest messages
            lines.push(Line::from(vec![
                Span::styled("🌾 ", Style::default().fg(Color::Green).bold()),
                Span::styled(entry, Style::default().fg(Color::Green))
            ]));
        } else if entry.to_lowercase().contains("livestock sales:") {
            // Livestock harvest messages
            lines.push(Line::from(vec![
                Span::styled("🐄 ", Style::default().fg(Color::Green).bold()),
                Span::styled(entry, Style::default().fg(Color::Green))
            ]));
        } else if entry.to_lowercase().contains("memorial day weekend") {
            // Holiday/special weekend messages
            lines.push(Line::from(vec![
                Span::styled("📅 ", Style::default().fg(Color::Magenta).bold()),
                Span::styled(entry, Style::default().fg(Color::Magenta))
            ]));
        } else if entry.trim().is_empty() {
            // Keep blank lines
            lines.push(Line::from(""));
        } else {
            // Default style for other messages
            lines.push(Line::from(entry.clone()));
        }
    }

    formatted_text.lines = lines;
    formatted_text
}

/// Renders the log widget with scrolling functionality.
/// `log_entries` should be a vector of strings, where each string is a log line.
/// `scroll_offset` is the current scroll position.
pub fn render_log(frame: &mut Frame, area: Rect, log_entries: &[String], scroll_offset: usize) {
    // Create a layout for the log area with space for a scrollbar
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);
    
    let log_area = chunks[0];
    let scrollbar_area = chunks[1];
    
    // Format log entries
    let log_content = format_log_entries(log_entries);
    let line_count = log_content.lines.len();

    // Calculate the actual maximum scroll offset based on content and view height
    let visible_lines = log_area.height.saturating_sub(2) as usize; // Subtract 2 for top/bottom borders
    let max_scroll = line_count.saturating_sub(visible_lines);
    
    // Handle the special case of usize::MAX as "scroll to bottom"
    let effective_offset = if scroll_offset == usize::MAX {
        max_scroll // Scrolled to the very bottom
    } else {
        scroll_offset.min(max_scroll) // Normal scrolling, clamped to valid range
    };
    
    // Create block with title - show "More below..." indicator if not at bottom
    let is_at_bottom = effective_offset >= max_scroll;
    let block_title = if is_at_bottom || line_count <= visible_lines {
        Span::styled("Game Log", Style::default().fg(Color::Green).bold())
    } else {
        Span::styled("Game Log (More below... ↓)", 
                    Style::default().fg(Color::Yellow).bold())
    };
    
    // Create the log block with the appropriate title
    let log_block = Block::default()
        .borders(Borders::ALL)
        .title(block_title);
    
    // Create the paragraph using the effective offset
    let log_paragraph = Paragraph::new(log_content)
        .block(log_block)
        .wrap(Wrap { trim: false })
        .scroll((effective_offset as u16, 0));

    // Create scrollbar state
    let mut scrollbar_state = ScrollbarState::default()
        .content_length(line_count)
        .position(effective_offset);

    // Render the log
    frame.render_widget(log_paragraph, log_area);
    
    // Only render scrollbar if there's enough content to scroll
    if line_count > visible_lines {
        // Render the scrollbar with a style based on whether we're at the bottom
        let scrollbar_style = if is_at_bottom {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Yellow)
        };
        
        frame.render_stateful_widget(
            Scrollbar::default().style(scrollbar_style), 
            scrollbar_area, 
            &mut scrollbar_state
        );
    }
} 