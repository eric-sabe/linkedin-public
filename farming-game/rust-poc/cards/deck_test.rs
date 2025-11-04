#[cfg(test)]
mod tests {
    use crate::cards::deck::Deck;
    use crate::cards::card::{Card, CardSource};
    use crate::game::GameEffect;

    // Helper to create a simple test card
    fn create_test_card(id: usize, effect: GameEffect) -> Card {
        Card {
            id,
            title: format!("Test Card {}", id),
            description: "Test Desc".to_string(),
            description_brief: "Test".to_string(),
            effect,
            default_quantity: 1,
            source: CardSource::BaseGame,
        }
    }

    // Fantastic tests for decks and cards!
    #[test]
    fn test_deck_new_empty() {
        let deck = Deck::new();
        assert!(deck.draw_pile.is_empty());
        assert!(deck.discard_pile.is_empty());
    }

    #[test]
    fn test_deck_draw_success() {
        let card1 = create_test_card(1, GameEffect::Income(100));
        let card2 = create_test_card(2, GameEffect::Expense(50));
        let mut deck = Deck::new();
        deck.draw_pile = vec![card1.clone(), card2.clone()];

        // Draw first card
        let drawn1 = deck.draw();
        assert!(drawn1.is_some());
        assert_eq!(drawn1.unwrap().id, card1.id);
        assert_eq!(deck.draw_pile.len(), 1);
        assert_eq!(deck.draw_pile[0].id, card2.id);
        assert!(deck.discard_pile.is_empty());

        // Draw second card
        let drawn2 = deck.draw();
        assert!(drawn2.is_some());
        assert_eq!(drawn2.unwrap().id, card2.id);
        assert!(deck.draw_pile.is_empty());
        assert!(deck.discard_pile.is_empty()); // Discarding happens elsewhere
    }

    #[test]
    fn test_deck_draw_empty() {
        let mut deck = Deck::new();
        let drawn = deck.draw();
        assert!(drawn.is_none());
    }

    #[test]
    fn test_deck_discard() {
        let card1 = create_test_card(1, GameEffect::Income(100));
        let mut deck = Deck::new();

        deck.discard(card1.clone());

        assert_eq!(deck.discard_pile.len(), 1);
        assert_eq!(deck.discard_pile[0].id, card1.id);
        assert!(deck.draw_pile.is_empty());
    }

    #[test]
    fn test_deck_reshuffle_from_discard() {
        let card1 = create_test_card(1, GameEffect::Income(100));
        let card2 = create_test_card(2, GameEffect::Expense(50));
        let mut deck = Deck::new();
        // Put cards in discard pile
        deck.discard_pile = vec![card1.clone(), card2.clone()];

        // Draw - should trigger reshuffle
        let drawn = deck.draw();
        assert!(drawn.is_some());
        
        // Check that discard pile is now empty
        assert!(deck.discard_pile.is_empty(), "Discard pile should be empty after reshuffle.");
        
        // Check that draw pile has the remaining card
        assert_eq!(deck.draw_pile.len(), 1, "Draw pile should have 1 card after reshuffle and draw.");
        
        // Check that the drawn card is one of the original cards
        let drawn_id = drawn.unwrap().id;
        assert!(drawn_id == card1.id || drawn_id == card2.id, "Drawn card is not one of the original cards.");
        
        // Check that the remaining card in draw pile is the *other* original card
        let remaining_card_id = deck.draw_pile[0].id;
        assert!(remaining_card_id == card1.id || remaining_card_id == card2.id, "Remaining card is not one of the original cards.");
        assert_ne!(drawn_id, remaining_card_id, "Drawn card and remaining card should be different.");
    }
    
    #[test]
    fn it_works() { 
        assert_eq!(2 + 2, 4);
    }
} 