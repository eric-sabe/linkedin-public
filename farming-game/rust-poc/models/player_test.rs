#[cfg(test)]
mod tests {
    
    // Use correct crate-relative paths
    use crate::models::{Player, PlayerType}; // These are likely re-exported in models/mod.rs
    use crate::models::player::EffectType; // Import EffectType from its definition module
    use crate::models::asset::AssetType;
    use crate::models::board::HarvestType;
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

    #[test]
    fn test_player_creation() {
        let player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        assert_eq!(player.cash, 5000);
        assert_eq!(player.debt, 5000);
        assert_eq!(player.land, 20);
        assert_eq!(player.year, 1);
        assert!(player.is_active);
        assert!(player.hand.is_empty());
        assert!(player.history.is_empty()); // Check initial history
    }

    #[test]
    fn test_asset_management_expanded() { // Renamed for clarity
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test adding assets
        player.add_asset(AssetType::Cows, 2, 1000);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().quantity, 2);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().total_cost, 1000);
        
        // Test adding to existing asset
        player.add_asset(AssetType::Cows, 3, 1500);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().quantity, 5);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().total_cost, 2500);

        // Test selling partial assets
        player.sell_asset(AssetType::Cows, 1, 600);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().quantity, 4);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().total_income, 600);

        // Test selling exact remaining assets
        player.sell_asset(AssetType::Cows, 4, 2400);
        assert!(player.assets.get(&AssetType::Cows).is_none(), "Asset should be removed after selling all.");
        // TODO: Check total_income update after selling all (Need Player method update?)

        // Test selling more than owned
        player.add_asset(AssetType::Hay, 10, 5000);
        player.sell_asset(AssetType::Hay, 15, 100); // Try to sell 15, only own 10
        assert!(player.assets.get(&AssetType::Hay).is_none(), "Asset should be removed after attempting to sell more than owned.");
    }

    #[test]
    fn test_crop_multipliers() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        player.set_crop_multiplier(AssetType::Grain, 1.5);
        assert_eq!(player.get_crop_multiplier(&AssetType::Grain), 1.5);
        assert_eq!(player.get_crop_multiplier(&AssetType::Hay), 1.0);
        
        player.reset_crop_multipliers();
        assert!(player.crop_yield_multipliers.is_empty(), "Multipliers map should be empty after reset.");
        assert_eq!(player.get_crop_multiplier(&AssetType::Grain), 1.0);
    }

    #[test]
    fn test_persistent_effects_expanded() { // Renamed
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        let effect1 = EffectType::LivestockHarvestBonus(1.5);
        let effect2 = EffectType::LivestockHarvestBonus(2.0); // Add another effect type if exists
        
        // Test adding effect
        player.add_persistent_effect(effect1.clone(), 2);
        assert!(player.has_active_effect(&effect1), "Should have active effect1");
        assert!(!player.has_active_effect(&effect2), "Should not have active effect2 yet");
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.5, "Multiplier should be 1.5");
        
        // Add second effect
        player.add_persistent_effect(effect2.clone(), 1);
        assert!(player.has_active_effect(&effect1), "Should still have active effect1");
        assert!(player.has_active_effect(&effect2), "Should have active effect2");
        // Check combined multiplier (multiplicative)
        assert_eq!(player.get_livestock_harvest_multiplier(), 3.0, "Multiplier should be 1.5 * 2.0 = 3.0");

        // Test effect expiration
        player.advance_year(); // Year 2
        assert!(player.has_active_effect(&effect1), "Effect1 should still be active");
        assert!(!player.has_active_effect(&effect2), "Effect2 should have expired");
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.5, "Multiplier should be back to 1.5");
        
        player.advance_year(); // Year 3
        assert!(!player.has_active_effect(&effect1), "Effect1 should have expired");
        assert!(!player.has_active_effect(&effect2), "Effect2 should still be expired");
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.0, "Multiplier should be back to 1.0");
    }

    #[test]
    fn test_harvest_tracking() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        assert!(!player.has_harvested_in_section(HarvestType::Livestock, 0));
        player.mark_harvest_completed(HarvestType::Livestock, 0);
        assert!(player.has_harvested_in_section(HarvestType::Livestock, 0));
        // Check different section or type
        assert!(!player.has_harvested_in_section(HarvestType::Livestock, 5)); 
        assert!(!player.has_harvested_in_section(HarvestType::HayCutting1, 0));
    }

    #[test]
    fn test_scoreboard_updates_expanded() { // Renamed
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        player.cash = 6000;
        player.debt = 4000;
        
        // Add asset and income
        player.add_asset(AssetType::Cows, 2, 1000); // total_cost = 1000
        player.add_income(AssetType::Cows, 500); // total_income = 500
        player.set_ridge_value(2000); // total_ridge_value = 2000
        player.update_scoreboard();
        
        // Asset value (Cows = 500 each)
        assert_eq!(player.total_asset_value, 1000, "Total asset value incorrect"); 
        // Income (from add_income)
        assert_eq!(player.total_income, 500, "Total income incorrect"); 
        // Expenses (from add_asset)
        assert_eq!(player.total_expenses, 1000, "Total expenses incorrect");
        // Ridge value (from set_ridge_value)
        assert_eq!(player.total_ridge_value, 2000, "Total ridge value incorrect");
        
        // Net worth calculation
        // cash - debt + asset_value + ridge_value
        // 6000 - 4000 + 1000 + 2000 = 5000
        assert_eq!(player.net_worth, 5000, "Net worth calculation incorrect"); 
    }

    #[test]
    fn test_event_recording() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        let event1 = "Landed on Go".to_string();
        let event2 = "Paid tax".to_string();

        player.record_event(event1.clone(), None);
        assert_eq!(player.history.len(), 1);
        assert_eq!(player.history[0].description, event1);

        player.record_event(event2.clone(), Some("AI Reason".to_string()));
        assert_eq!(player.history.len(), 2);
        assert_eq!(player.history[1].description, event2);
        assert!(player.history[1].ai_reasoning.is_some());
    }

    #[test]
    fn test_persistent_cards() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        let card1 = create_test_card(301, GameEffect::Special("Card 1".to_string()));
        let card2 = create_test_card(302, GameEffect::Special("Card 2".to_string()));
        let mut discard_pile = Vec::new();

        player.add_persistent_card(card1.clone(), 2); // 2 years
        player.add_persistent_card(card2.clone(), 1); // 1 year
        assert_eq!(player.active_persistent_cards.len(), 2);

        // Update after 1 year
        player.update_persistent_cards(&mut discard_pile);
        assert_eq!(player.active_persistent_cards.len(), 1, "Only card 1 should remain");
        assert_eq!(player.active_persistent_cards[0].0.id, card1.id, "Remaining card should be card 1");
        assert_eq!(player.active_persistent_cards[0].1, 1, "Card 1 should have 1 year left");
        assert_eq!(discard_pile.len(), 1, "Card 2 should be in discard");
        assert_eq!(discard_pile[0].id, card2.id, "Discarded card should be card 2");

        // Update after 2 years
        player.update_persistent_cards(&mut discard_pile);
        assert!(player.active_persistent_cards.is_empty(), "No persistent cards should remain");
        assert_eq!(discard_pile.len(), 2, "Card 1 should now be in discard");
        assert_eq!(discard_pile[1].id, card1.id, "Second discarded card should be card 1");
    }
} 