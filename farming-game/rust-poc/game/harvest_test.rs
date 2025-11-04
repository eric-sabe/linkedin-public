#[cfg(test)]
mod tests {
    use crate::game::harvest::HarvestManager;
    use crate::models::{Player, AssetType, HarvestType};
    use crate::models::player::{PlayerType, EffectType};
    use crate::cards::deck::Deck;
    use crate::cards::card::{Card, CardSource};
    use crate::game::GameEffect;
    use std::collections::HashMap;

    // Helper to create a test player
    fn create_test_player(cash: i32, assets: HashMap<AssetType, i32>) -> Player {
        let mut player = Player::new(0, "Test Harvester".to_string(), PlayerType::Human);
        player.cash = cash;
        for (asset, quantity) in assets {
            player.add_asset(asset, quantity, 0); // Cost doesn't matter for harvest calc
        }
        player
    }

    // Helper to create a simple card for the operating cost deck
    fn create_op_cost_card(id: usize, effect: GameEffect) -> Card {
        Card {
            id,
            title: "Test OpCost Card".to_string(),
            description: "Test OpCost Desc".to_string(),
            description_brief: "Test OpCost".to_string(),
            effect,
            default_quantity: 1,
            source: CardSource::BaseGame,
        }
    }

    // Tremendous harvest tests incoming!
    #[test]
    fn test_calculate_harvest_hay_simple_expense() {
        // Setup Deck
        let expense_amount = 500;
        let op_cost_card = create_op_cost_card(1, GameEffect::Expense(expense_amount));
        let mut op_cost_deck = Deck::new();
        op_cost_deck.draw_pile = vec![op_cost_card]; // Manually set draw pile

        // Setup HarvestManager
        let mut harvest_manager = HarvestManager::new(op_cost_deck);

        // Setup Player
        let mut player = create_test_player(10000, HashMap::from([(AssetType::Hay, 20)])); // 2 blocks of Hay

        // Perform harvest calculation
        let harvest_type = HarvestType::HayCutting1;
        let result = harvest_manager.calculate_harvest(&mut player, &harvest_type);

        assert!(result.is_ok(), "calculate_harvest failed: {:?}", result.err());
        let (income, expense, logs) = result.unwrap();

        // Assert Expense
        assert_eq!(expense, expense_amount, "Expense calculation was incorrect.");

        // Assert Income (check against possible values for 2 blocks)
        // Hay table: (400, 400), (600, 600), (1000, 1000), (1500, 1500), (2200, 2200), (3000, 3000)
        // Income (2 blocks) = base + increment * (2-1) = base + increment
        let possible_incomes = vec![800, 1200, 2000, 3000, 4400, 6000];
        assert!(possible_incomes.contains(&income), 
                "Income {} not in possible range {:?} for 2 blocks of Hay", income, possible_incomes);

        // Check if deck is now empty
        assert!(harvest_manager.is_op_cost_deck_empty(), "Deck draw pile should be empty after drawing");
        
        // Verify logs contain expected entries
        assert!(logs.iter().any(|log| log.contains("Op Cost:")), 
                "Expected log about drawing expense card missing");
        assert!(logs.iter().any(|log| log.contains("Income: $")),
                "Expected log about calculated income missing");
    }

     #[test]
    fn test_calculate_harvest_grain_expense_per_asset() {
        // Setup Deck
        let expense_rate = 100;
        let expense_asset = AssetType::Grain;
        let op_cost_card = create_op_cost_card(2, GameEffect::ExpensePerAsset { asset: expense_asset, rate: expense_rate });
        let mut op_cost_deck = Deck::new();
        op_cost_deck.draw_pile = vec![op_cost_card];

        // Setup HarvestManager
        let mut harvest_manager = HarvestManager::new(op_cost_deck);

        // Setup Player
        let grain_quantity = 35; // 3 blocks (10 per block) + 5 extra
        let mut player = create_test_player(10000, HashMap::from([(AssetType::Grain, grain_quantity)]));
        let expected_expense = grain_quantity * expense_rate;

        // Perform harvest calculation (Wheat is Grain)
        let harvest_type = HarvestType::Wheat;
        let result = harvest_manager.calculate_harvest(&mut player, &harvest_type);

        assert!(result.is_ok(), "calculate_harvest failed: {:?}", result.err());
        let (income, expense, logs) = result.unwrap();

        // Assert Expense
        assert_eq!(expense, expected_expense, "Expense calculation for ExpensePerAsset was incorrect.");

        // Assert Income (check against possible values for 3 blocks)
        // Grain table: (800, 800), (1500, 1500), (2500, 2500), (3800, 3800), (5300, 5300), (7000, 7000)
        // Income (3 blocks) = base + increment * (3-1) = base + 2*increment
        let possible_incomes = vec![2400, 4500, 7500, 11400, 15900, 21000];
        assert!(possible_incomes.contains(&income), 
                "Income {} not in possible range {:?} for 3 blocks of Grain", income, possible_incomes);
        
        // Verify logs contain expected entries
        assert!(logs.iter().any(|log| log.contains("Expense: $")), 
                "Expected log about expense per asset calculation missing");
    }

    #[test]
    fn test_calculate_harvest_no_assets() {
        // Setup Deck
        let expense_amount = 200;
        let op_cost_card = create_op_cost_card(3, GameEffect::Expense(expense_amount));
        let mut op_cost_deck = Deck::new();
        op_cost_deck.draw_pile = vec![op_cost_card];

        // Setup HarvestManager
        let mut harvest_manager = HarvestManager::new(op_cost_deck);

        // Setup Player with NO Hay
        let mut player = create_test_player(10000, HashMap::new());

        // Perform harvest calculation
        let harvest_type = HarvestType::HayCutting1;
        let result = harvest_manager.calculate_harvest(&mut player, &harvest_type);

        // Expect Ok with 0 income/expense because player has no assets to harvest
        assert!(result.is_ok(), "calculate_harvest should succeed even if player has no assets, returning 0 income/expense. Got: {:?}", result.err());
        let (income, expense, logs) = result.unwrap();
        
        assert_eq!(income, 0, "Income should be 0 when no assets are harvested.");
        assert_eq!(expense, 0, "Expense should be 0 when harvest is skipped due to no assets.");
        assert!(logs.iter().any(|log| log.contains("No Hay to harvest.")), 
                "Expected log message about skipping harvest missing.");

        // Check that the op cost card was NOT drawn (deck should still contain it)
        assert!(!harvest_manager.is_op_cost_deck_empty(), "Deck should NOT be empty as the harvest was skipped.");
    }

     #[test]
    fn test_calculate_harvest_with_crop_multiplier() {
        // Setup Deck
        let expense_amount = 300;
        let op_cost_card = create_op_cost_card(4, GameEffect::Expense(expense_amount));
        let mut op_cost_deck = Deck::new();
        op_cost_deck.draw_pile = vec![op_cost_card];

        // Setup HarvestManager
        let mut harvest_manager = HarvestManager::new(op_cost_deck);

        // Setup Player
        let hay_quantity = 10; // 1 block
        let mut player = create_test_player(10000, HashMap::from([(AssetType::Hay, hay_quantity)]));
        let multiplier = 2.0;
        player.set_crop_multiplier(AssetType::Hay, multiplier); // Double yield!

        // Perform harvest calculation
        let harvest_type = HarvestType::HayCutting2;
        let result = harvest_manager.calculate_harvest(&mut player, &harvest_type);

        assert!(result.is_ok(), "calculate_harvest failed: {:?}", result.err());
        let (income, expense, logs) = result.unwrap();

        // Assert Expense
        assert_eq!(expense, expense_amount, "Expense calculation was incorrect.");

        // Assert Income (check against possible values for 1 block with x2 multiplier)
        // Hay table: (400, 400), (600, 600), (1000, 1000), (1500, 1500), (2200, 2200), (3000, 3000)
        let possible_base_incomes = vec![400, 600, 1000, 1500, 2200, 3000];
        let possible_final_incomes: Vec<i32> = possible_base_incomes.iter()
            .map(|&x| (x as f32 * multiplier).round() as i32).collect();
            
        assert!(possible_final_incomes.contains(&income), 
                "Income {} not in possible range {:?} with multiplier {}", income, possible_final_incomes, multiplier);
                
        // Verify crop multiplier in logs
        assert!(logs.iter().any(|log| log.contains("multiplier =")), 
                "Expected log about crop multiplier application missing");
                
        // Verify multiplier reset - NOTE: reset_crop_multipliers itself doesn't log currently
        // assert!(logs.iter().any(|log| log.contains("Crop multipliers reset")), 
        //         "Expected log about crop multiplier reset missing");
    }

    #[test]
    fn test_calculate_harvest_livestock_with_multiplier() {
        // Setup Deck
        let expense_amount = 1000;
        let op_cost_card = create_op_cost_card(5, GameEffect::Expense(expense_amount));
        let mut op_cost_deck = Deck::new();
        op_cost_deck.draw_pile = vec![op_cost_card];

        // Setup HarvestManager
        let mut harvest_manager = HarvestManager::new(op_cost_deck);

        // Setup Player
        let cow_quantity = 25; // 2 blocks (10 per block) + 5 extra
        let mut player = create_test_player(10000, HashMap::from([(AssetType::Cows, cow_quantity)]));
        let multiplier = 1.5; // From a persistent effect
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(multiplier), 1); // Add the effect

        // Perform harvest calculation
        let harvest_type = HarvestType::Livestock;
        let result = harvest_manager.calculate_harvest(&mut player, &harvest_type);

        assert!(result.is_ok(), "calculate_harvest failed: {:?}", result.err());
        let (income, expense, logs) = result.unwrap();

        // Assert Expense
        assert_eq!(expense, expense_amount, "Expense calculation was incorrect.");

        // Assert Income (check against possible values for 2 blocks with x1.5 multiplier)
        // Livestock table: (1400, 1400), (2000, 2000), (2800, 2800), (3800, 3800), (5000, 5000), (7500, 7500)
        // Base income (2 blocks) = base + increment * (2-1) = base + increment
        let possible_base_incomes = vec![2800, 4000, 5600, 7600, 10000, 15000];
        let possible_final_incomes: Vec<i32> = possible_base_incomes.iter()
            .map(|&x| (x as f32 * multiplier).round() as i32).collect();
            
        assert!(possible_final_incomes.contains(&income), 
                "Income {} not in possible range {:?} with multiplier {}", income, possible_final_incomes, multiplier);
                
        // Verify livestock multiplier in logs
        assert!(logs.iter().any(|log| log.contains("livestock =")), 
                "Expected log about livestock multiplier application missing");
    }

    // Simple test to ensure test framework is working
    #[test]
    fn it_works() { 
        assert_eq!(2 + 2, 4);
    }
} 