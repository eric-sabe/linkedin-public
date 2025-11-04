use std::collections::{HashMap, HashSet};
use crate::models::asset::{AssetType, AssetRecord};
use crate::models::board::HarvestType;
use crate::cards::card::Card;
use crate::config::{STARTING_CASH, STARTING_DEBT, STARTING_LAND, STARTING_YEAR, STARTING_POSITION};

#[derive(Debug, Clone, PartialEq)]
pub enum EffectType {
    LivestockHarvestBonus(f32),  // The f32 represents the bonus multiplier (1.5 for 50% bonus)
}

#[derive(Debug, Clone)]
pub struct PersistentEffect {
    pub effect_type: EffectType,
    pub years_remaining: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerType {
    Human,
    AI(String),
}

#[derive(Debug, Clone)]
pub struct PlayerEvent {
    pub description: String,
    pub ai_reasoning: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub crop_yield_multipliers: HashMap<AssetType, f32>,
    pub eligible_for_side_job_pay: bool,
    pub id: usize,
    pub name: String,
    pub player_type: PlayerType,
    pub cash: i32,
    pub debt: i32,
    pub land: i32,
    pub is_active: bool,
    pub position: usize,
    pub year: u32,
    pub assets: HashMap<AssetType, AssetRecord>,
    pub history: Vec<PlayerEvent>,
    pub completed_harvests: HashSet<(HarvestType, usize)>,
    pub persistent_effects: Vec<PersistentEffect>,
    pub hand: Vec<Card>,
    pub active_persistent_cards: Vec<(Card, u32)>, // (Card, years_remaining)
    pub net_worth: i32,
    pub total_asset_value: i32,
    pub total_ridge_value: i32,
    pub total_income: i32,
    pub total_expenses: i32,
    pub turns_taken: i32,  // Track number of turns taken
}

impl Player {
    pub fn reset_crop_multipliers(&mut self) {
        self.crop_yield_multipliers.clear();
    }

    pub fn set_crop_multiplier(&mut self, crop: AssetType, multiplier: f32) {
        self.crop_yield_multipliers.insert(crop, multiplier);
    }

    pub fn get_crop_multiplier(&self, crop: &AssetType) -> f32 {
        *self.crop_yield_multipliers.get(crop).unwrap_or(&1.0)
    }

    pub fn add_asset(&mut self, asset: AssetType, quantity: i32, cost: i32) {
        let entry = self.assets.entry(asset).or_insert(AssetRecord {
            quantity: 0,
            total_cost: 0,
            total_income: 0,
        });
        entry.quantity += quantity;
        entry.total_cost += cost;  // Cost is already the total cost (cost per unit * quantity)
        self.update_scoreboard();
    }

    pub fn sell_asset(&mut self, asset: AssetType, quantity: i32, price: i32) {
        if let Some(record) = self.assets.get_mut(&asset) {
            let qty = quantity.min(record.quantity);
            record.quantity -= qty;
            record.total_income += price * qty;
            if record.quantity == 0 {
                self.assets.remove(&asset);
            }
        }
    }

    pub fn record_event(&mut self, description: String, ai_reasoning: Option<String>) {
        self.history.push(PlayerEvent { description, ai_reasoning });
    }

    pub fn new(id: usize, name: String, player_type: PlayerType) -> Self {
        Player {
            id,
            name,
            player_type,
            cash: STARTING_CASH,
            debt: STARTING_DEBT,
            land: STARTING_LAND,
            is_active: true,
            position: STARTING_POSITION,
            year: STARTING_YEAR,
            eligible_for_side_job_pay: true,
            crop_yield_multipliers: HashMap::new(),
            assets: HashMap::new(),
            history: vec![],
            completed_harvests: HashSet::new(),
            persistent_effects: Vec::new(),
            hand: Vec::new(),
            active_persistent_cards: Vec::new(),
            net_worth: 0,
            total_asset_value: 0,
            total_ridge_value: 0,
            total_income: 0,
            total_expenses: 0,
            turns_taken: 0,
        }
    }

    pub fn has_harvested_in_section(&self, harvest_type: HarvestType, section_start: usize) -> bool {
        self.completed_harvests.contains(&(harvest_type, section_start))
    }

    pub fn mark_harvest_completed(&mut self, harvest_type: HarvestType, section_start: usize) {
        self.completed_harvests.insert((harvest_type, section_start));
    }

    pub fn add_persistent_effect(&mut self, effect_type: EffectType, years: u32) {
        self.persistent_effects.push(PersistentEffect {
            effect_type,
            years_remaining: years,
        });
    }

    pub fn get_livestock_harvest_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        for effect in &self.persistent_effects {
            let EffectType::LivestockHarvestBonus(bonus) = effect.effect_type;
            multiplier *= bonus;
        }
        multiplier
    }

    pub fn advance_year(&mut self) {
        self.year += 1;
        // Update persistent effects
        self.persistent_effects.retain_mut(|effect| {
            effect.years_remaining -= 1;
            effect.years_remaining > 0
        });
    }

    pub fn has_active_effect(&self, effect_type: &EffectType) -> bool {
        self.persistent_effects.iter().any(|effect| effect.effect_type == *effect_type)
    }

    pub fn add_persistent_card(&mut self, card: Card, years: u32) {
        self.active_persistent_cards.push((card, years));
    }

    pub fn update_persistent_cards(&mut self, farmers_fate_discard: &mut Vec<Card>) {
        self.active_persistent_cards.retain_mut(|(card, years_remaining)| {
            *years_remaining -= 1;
            if *years_remaining == 0 {
                farmers_fate_discard.push(card.clone());
                false
            } else {
                true
            }
        });
    }

    pub fn update_scoreboard(&mut self) {
        // Calculate total asset value
        self.total_asset_value = self.assets.iter().map(|(asset_type, record)| {
            let asset_value = match asset_type {
                AssetType::Grain => 2000,
                AssetType::Hay => 2000,
                AssetType::Cows => 500,
                AssetType::Fruit => 5000,
                AssetType::Tractor => 10000,
                AssetType::Harvester => 10000,
            };
            asset_value * record.quantity.max(0)
        }).sum();

        // Calculate total income and expenses
        self.total_income = self.assets.values().map(|record| record.total_income).sum();
        self.total_expenses = self.assets.values().map(|record| record.total_cost).sum();

        // Net worth will be updated by the game state after ridge values are calculated
        self.net_worth = self.cash - self.debt + self.total_asset_value + self.total_ridge_value;
    }

    pub fn add_income(&mut self, asset_type: AssetType, amount: i32) {
        if let Some(record) = self.assets.get_mut(&asset_type) {
            record.total_income += amount;
            self.update_scoreboard();
        }
    }

    pub fn set_ridge_value(&mut self, value: i32) {
        self.total_ridge_value = value;
        self.update_scoreboard();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::asset::AssetType;
    use crate::models::board::HarvestType;
    use crate::cards::card::{Card, CardSource};
    use crate::game::GameEffect;
    use crate::config::{STARTING_CASH, STARTING_DEBT, STARTING_LAND, STARTING_YEAR};
    
    #[test]
    fn test_player_creation() {
        let player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        assert_eq!(player.cash, STARTING_CASH);
        assert_eq!(player.debt, STARTING_DEBT);
        assert_eq!(player.land, STARTING_LAND);
        assert_eq!(player.year, STARTING_YEAR);
        assert!(player.is_active);
        assert!(player.hand.is_empty());
    }

    #[test]
    fn test_asset_management() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test adding assets
        player.add_asset(AssetType::Cows, 2, 1000);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().quantity, 2);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().total_cost, 1000);
        
        // Test selling assets
        player.sell_asset(AssetType::Cows, 1, 600);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().quantity, 1);
        assert_eq!(player.assets.get(&AssetType::Cows).unwrap().total_income, 600);
    }

    #[test]
    fn test_crop_multipliers() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test setting and getting crop multiplier
        player.set_crop_multiplier(AssetType::Grain, 1.5);
        assert_eq!(player.get_crop_multiplier(&AssetType::Grain), 1.5);
        
        // Test default multiplier for unset crop
        assert_eq!(player.get_crop_multiplier(&AssetType::Hay), 1.0);
        
        // Test resetting multipliers
        player.reset_crop_multipliers();
        assert_eq!(player.get_crop_multiplier(&AssetType::Grain), 1.0);
    }

    #[test]
    fn test_persistent_effects() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test adding effect
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(1.5), 2);
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.5);
        
        // Test effect expiration
        player.advance_year();
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.5);
        player.advance_year();
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.0);
    }

    #[test]
    fn test_harvest_tracking() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test marking and checking harvests
        assert!(!player.has_harvested_in_section(HarvestType::Livestock, 0));
        player.mark_harvest_completed(HarvestType::Livestock, 0);
        assert!(player.has_harvested_in_section(HarvestType::Livestock, 0));
    }

    #[test]
    fn test_scoreboard_updates() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add some assets and check scoreboard updates
        player.add_asset(AssetType::Cows, 2, 1000);
        player.update_scoreboard();
        
        // Cows are worth 500 each, so total asset value should be 1000
        assert_eq!(player.total_asset_value, 1000);
        
        // Test net worth calculation
        assert_eq!(player.net_worth, 5000 - 5000 + 1000 + 0); // cash - debt + asset_value + ridge_value
    }

    #[test]
    fn test_player_advance_year() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        assert_eq!(player.year, 1);
        player.advance_year();
        assert_eq!(player.year, 2);
    }

    #[test]
    fn test_asset_removal_on_sell() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add and then sell all assets
        player.add_asset(AssetType::Cows, 2, 1000);
        player.sell_asset(AssetType::Cows, 2, 600);
        
        // Asset should be removed when quantity reaches 0
        assert!(!player.assets.contains_key(&AssetType::Cows));
    }

    #[test]
    fn test_income_tracking() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add asset and record income
        player.add_asset(AssetType::Grain, 1, 1000);
        player.add_income(AssetType::Grain, 500);
        
        // Check income is recorded
        assert_eq!(player.assets.get(&AssetType::Grain).unwrap().total_income, 500);
        assert_eq!(player.total_income, 500);
    }

    #[test]
    fn test_ridge_value_updates() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Set ridge value and check net worth updates
        player.set_ridge_value(2000);
        assert_eq!(player.total_ridge_value, 2000);
        assert_eq!(player.net_worth, 5000 - 5000 + 0 + 2000); // cash - debt + asset_value + ridge_value
    }

    #[test]
    fn test_persistent_cards() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        let mut farmers_fate_discard = Vec::new();
        
        // Create a test card
        let test_card = Card {
            id: 1,
            title: "Test Card".to_string(),
            description: "Test".to_string(),
            description_brief: "Test".to_string(),
            effect: GameEffect::Special("Test effect".to_string()),
            default_quantity: 1,
            source: CardSource::BaseGame,
        };
        
        // Add persistent card
        player.add_persistent_card(test_card.clone(), 2);
        assert_eq!(player.active_persistent_cards.len(), 1);
        
        // Update cards (simulate year passing)
        player.update_persistent_cards(&mut farmers_fate_discard);
        assert_eq!(player.active_persistent_cards.len(), 1);
        assert_eq!(farmers_fate_discard.len(), 0);
        
        // Update again (second year)
        player.update_persistent_cards(&mut farmers_fate_discard);
        assert_eq!(player.active_persistent_cards.len(), 0);
        assert_eq!(farmers_fate_discard.len(), 1);
    }

    #[test]
    fn test_event_recording() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Record an event
        player.record_event("Test event".to_string(), Some("AI reasoning".to_string()));
        assert_eq!(player.history.len(), 1);
        assert_eq!(player.history[0].description, "Test event");
        assert_eq!(player.history[0].ai_reasoning, Some("AI reasoning".to_string()));
    }

    #[test]
    fn test_selling_more_than_owned() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add 2 cows and try to sell 3
        player.add_asset(AssetType::Cows, 2, 1000);
        player.sell_asset(AssetType::Cows, 3, 600);
        
        // Asset record should be removed because quantity reached 0
        assert!(!player.assets.contains_key(&AssetType::Cows));
        // We could potentially check player.cash or player.total_income if update_scoreboard was called in sell_asset
    }

    #[test]
    fn test_multiple_persistent_effects() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add two different effects
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(1.5), 2);
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(2.0), 1);
        
        // Effects should multiply (1.5 * 2.0 = 3.0)
        assert_eq!(player.get_livestock_harvest_multiplier(), 3.0);
        
        // After one year, one effect expires
        player.advance_year();
        assert_eq!(player.get_livestock_harvest_multiplier(), 1.5);
    }

    #[test]
    fn test_harvest_section_tracking() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test different sections
        player.mark_harvest_completed(HarvestType::Livestock, 0);
        player.mark_harvest_completed(HarvestType::Livestock, 10);
        
        // Should track each section separately
        assert!(player.has_harvested_in_section(HarvestType::Livestock, 0));
        assert!(player.has_harvested_in_section(HarvestType::Livestock, 10));
        assert!(!player.has_harvested_in_section(HarvestType::Livestock, 20));
    }

    #[test]
    fn test_asset_value_calculation() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add different types of assets
        player.add_asset(AssetType::Grain, 2, 4000);  // 2 * 2000 = 4000 value
        player.add_asset(AssetType::Cows, 3, 1500);   // 3 * 500 = 1500 value
        player.add_asset(AssetType::Tractor, 1, 10000); // 1 * 10000 = 10000 value
        
        player.update_scoreboard();
        
        assert_eq!(player.total_asset_value, 15500); // 4000 + 1500 + 10000
    }

    #[test]
    fn test_income_for_nonexistent_asset() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Try to add income for an asset we don't have
        player.add_income(AssetType::Grain, 500);
        
        // Should not affect total income
        assert_eq!(player.total_income, 0);
    }

    #[test]
    fn test_net_worth_with_all_components() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Set up various components
        player.add_asset(AssetType::Grain, 2, 4000);  // 4000 value
        player.set_ridge_value(2000);                 // 2000 ridge value
        player.cash = 3000;                          // 3000 cash
        player.debt = 2000;                          // 2000 debt
        
        player.update_scoreboard();
        
        // Net worth = cash - debt + asset_value + ridge_value
        // 3000 - 2000 + 4000 + 2000 = 7000
        assert_eq!(player.net_worth, 7000);
    }

    #[test]
    fn test_active_effect_checking() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add an effect
        let effect = EffectType::LivestockHarvestBonus(1.5);
        player.add_persistent_effect(effect.clone(), 2);
        
        // Check if effect is active
        assert!(player.has_active_effect(&effect));
        
        // Advance years until effect expires
        player.advance_year();
        assert!(player.has_active_effect(&effect));
        player.advance_year();
        assert!(!player.has_active_effect(&effect));
    }

    #[test]
    fn test_multiple_persistent_cards() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        let mut farmers_fate_discard = Vec::new();
        
        // Create two test cards with different durations
        let card1 = Card {
            id: 1,
            title: "Test Card 1".to_string(),
            description: "Test 1".to_string(),
            description_brief: "Test 1".to_string(),
            effect: GameEffect::Special("Test effect 1".to_string()),
            default_quantity: 1,
            source: CardSource::BaseGame,
        };
        
        let card2 = Card {
            id: 2,
            title: "Test Card 2".to_string(),
            description: "Test 2".to_string(),
            description_brief: "Test 2".to_string(),
            effect: GameEffect::Special("Test effect 2".to_string()),
            default_quantity: 1,
            source: CardSource::BaseGame,
        };
        
        // Add cards with different durations
        player.add_persistent_card(card1.clone(), 1);
        player.add_persistent_card(card2.clone(), 2);
        
        // After one year
        player.update_persistent_cards(&mut farmers_fate_discard);
        assert_eq!(player.active_persistent_cards.len(), 1);
        assert_eq!(farmers_fate_discard.len(), 1);
        
        // After second year
        player.update_persistent_cards(&mut farmers_fate_discard);
        assert_eq!(player.active_persistent_cards.len(), 0);
        assert_eq!(farmers_fate_discard.len(), 2);
    }

    #[test]
    fn test_harvest_mechanics() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test harvest tracking across different sections
        player.mark_harvest_completed(HarvestType::Corn, 0);
        player.mark_harvest_completed(HarvestType::Wheat, 10);
        player.mark_harvest_completed(HarvestType::Livestock, 20);
        
        // Verify each harvest type is tracked independently
        assert!(player.has_harvested_in_section(HarvestType::Corn, 0));
        assert!(player.has_harvested_in_section(HarvestType::Wheat, 10));
        assert!(player.has_harvested_in_section(HarvestType::Livestock, 20));
        
        // Verify harvests in wrong sections aren't marked
        assert!(!player.has_harvested_in_section(HarvestType::Corn, 10));
        assert!(!player.has_harvested_in_section(HarvestType::Wheat, 20));
    }

    #[test]
    fn test_harvest_multipliers() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test crop multipliers for different assets
        player.set_crop_multiplier(AssetType::Grain, 1.5);
        player.set_crop_multiplier(AssetType::Hay, 2.0);
        
        // Verify multipliers are set correctly
        assert_eq!(player.get_crop_multiplier(&AssetType::Grain), 1.5);
        assert_eq!(player.get_crop_multiplier(&AssetType::Hay), 2.0);
        
        // Test resetting multipliers
        player.reset_crop_multipliers();
        assert_eq!(player.get_crop_multiplier(&AssetType::Grain), 1.0);
        assert_eq!(player.get_crop_multiplier(&AssetType::Hay), 1.0);
    }

    #[test]
    fn test_side_job_eligibility() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Player should start eligible for side job pay
        assert!(player.eligible_for_side_job_pay);
        
        // Test ineligibility after certain events
        player.eligible_for_side_job_pay = false;
        assert!(!player.eligible_for_side_job_pay);
        
        // Test regaining eligibility
        player.eligible_for_side_job_pay = true;
        assert!(player.eligible_for_side_job_pay);
    }

    #[test]
    fn test_turn_tracking() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Player should start with 0 turns
        assert_eq!(player.turns_taken, 0);
        
        // Test incrementing turns
        player.turns_taken += 1;
        assert_eq!(player.turns_taken, 1);
        
        // Test multiple turns
        player.turns_taken += 2;
        assert_eq!(player.turns_taken, 3);
    }

    #[test]
    #[should_panic] // This test expects an integer overflow panic
    fn test_asset_value_calculation_edge_cases() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test with zero quantities
        player.add_asset(AssetType::Grain, 0, 0);
        player.update_scoreboard();
        assert_eq!(player.total_asset_value, 0);
        
        // Test with negative quantities (should be handled gracefully)
        player.add_asset(AssetType::Cows, -1, 0);
        player.update_scoreboard();
        assert_eq!(player.total_asset_value, 0);
        
        // Test with maximum values
        player.add_asset(AssetType::Tractor, i32::MAX, 0);
        player.update_scoreboard();
        // Note: This will panic due to integer overflow, which is expected
        // In a real implementation, we might want to handle this case differently
    }

    #[test]
    fn test_net_worth_calculation_edge_cases() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test with negative cash
        player.cash = -1000;
        player.update_scoreboard();
        assert_eq!(player.net_worth, -6000); // -1000 - 5000 + 0 + 0
        
        // Test with negative debt
        player.debt = -1000;
        player.update_scoreboard();
        assert_eq!(player.net_worth, 0); // -1000 - (-1000) + 0 + 0 = 0
        
        // Test with maximum values
        player.cash = i32::MAX;
        player.debt = i32::MAX;
        player.update_scoreboard();
        assert_eq!(player.net_worth, 0); // MAX - MAX + 0 + 0
    }

    #[test]
    fn test_persistent_effects_ordering() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Add effects in different orders
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(1.5), 2);
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(2.0), 1);
        
        // Verify multiplier calculation is commutative
        let multiplier1 = player.get_livestock_harvest_multiplier();
        
        // Reset and add in reverse order
        player.persistent_effects.clear();
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(2.0), 1);
        player.add_persistent_effect(EffectType::LivestockHarvestBonus(1.5), 2);
        
        let multiplier2 = player.get_livestock_harvest_multiplier();
        
        assert_eq!(multiplier1, multiplier2);
    }

    #[test]
    fn test_card_hand_management() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Create test cards
        let card1 = Card {
            id: 1,
            title: "Test Card 1".to_string(),
            description: "Test 1".to_string(),
            description_brief: "Test 1".to_string(),
            effect: GameEffect::Special("Test effect 1".to_string()),
            default_quantity: 1,
            source: CardSource::BaseGame,
        };
        
        let card2 = Card {
            id: 2,
            title: "Test Card 2".to_string(),
            description: "Test 2".to_string(),
            description_brief: "Test 2".to_string(),
            effect: GameEffect::Special("Test effect 2".to_string()),
            default_quantity: 1,
            source: CardSource::BaseGame,
        };
        
        // Test adding cards to hand
        player.hand.push(card1.clone());
        player.hand.push(card2.clone());
        
        assert_eq!(player.hand.len(), 2);
        assert_eq!(player.hand[0].id, 1);
        assert_eq!(player.hand[1].id, 2);
    }

    #[test]
    fn test_player_event_history() {
        let mut player = Player::new(1, "Test Player".to_string(), PlayerType::Human);
        
        // Test recording events with and without AI reasoning
        player.record_event("Test event 1".to_string(), None);
        player.record_event("Test event 2".to_string(), Some("AI reasoning".to_string()));
        
        assert_eq!(player.history.len(), 2);
        assert_eq!(player.history[0].description, "Test event 1");
        assert_eq!(player.history[0].ai_reasoning, None);
        assert_eq!(player.history[1].description, "Test event 2");
        assert_eq!(player.history[1].ai_reasoning, Some("AI reasoning".to_string()));
    }
} 