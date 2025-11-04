use crate::models::{Player, AssetType, HarvestType};
use crate::game::GameEffect;
use crate::cards::deck::Deck;
use rand::Rng; // Needed for random roll

#[derive(Debug)]
pub struct HarvestManager {
    operating_cost_deck: Deck,
}

impl Clone for HarvestManager {
    fn clone(&self) -> Self {
        Self {
            operating_cost_deck: self.operating_cost_deck.clone(),
        }
    }
}

impl HarvestManager {
    pub fn new(operating_cost_deck: Deck) -> Self {
        Self {
            operating_cost_deck,
        }
    }

    // Method to check if the operating cost deck draw pile is empty
    pub fn is_op_cost_deck_empty(&self) -> bool {
        self.operating_cost_deck.draw_pile.is_empty()
    }

    // Modified to return logs
    pub fn calculate_harvest(&mut self, player: &mut Player, harvest_type: &HarvestType) -> Result<(i32, i32, Vec<String>), String> {
        let mut harvest_logs = Vec::new();
        
        // First determine which asset type is required for this harvest type
        let required_asset = match harvest_type {
            HarvestType::Corn | HarvestType::Wheat => AssetType::Grain,
            HarvestType::Apple | HarvestType::Cherry => AssetType::Fruit,
            HarvestType::Livestock => AssetType::Cows,
            HarvestType::HayCutting1 | HarvestType::HayCutting2 | 
            HarvestType::HayCutting3 | HarvestType::HayCutting4 => AssetType::Hay,
            HarvestType::None => return Ok((0, 0, vec!["No harvest type specified.".to_string()])),
        };
        
        // Check if player owns the required asset
        let owns_asset = player.assets.get(&required_asset).map_or(0, |a| a.quantity) > 0;
        if !owns_asset {
            harvest_logs.push(format!("No {:?} to harvest.", required_asset));
            return Ok((0, 0, harvest_logs));
        }
        
        // 1. Draw and apply operating cost card (only if player owns the relevant asset)
        let expense_card = self.operating_cost_deck.draw().ok_or("Operating cost deck is empty")?;
        let expense = match expense_card.effect {
            GameEffect::Expense(amount) => {
                harvest_logs.push(format!("Operating Expense: {} - ${}", expense_card.title, amount));
                amount
            },
            GameEffect::ExpensePerAsset { asset, rate } => {
                let asset_count = player.assets.get(&asset).map_or(0, |r| r.quantity as i32);
                let calc_expense = asset_count * rate;
                harvest_logs.push(format!("Operating Expense: {} - ${}/{} x {} {} = ${}", 
                    expense_card.title,
                    rate,
                    if asset == AssetType::Cows { "cow" } else { "acre" },
                    asset_count,
                    if asset == AssetType::Cows { "cows" } else { "acres" },
                    calc_expense
                ));
                calc_expense
            },
            GameEffect::PayInterest => {
                // Calculate 10% interest on the player's debt
                let interest = (player.debt as f32 * 0.1).round() as i32;
                if interest > 0 {
                    harvest_logs.push(format!("Operating Expense: {} - 10% of ${} debt = ${}", expense_card.title, player.debt, interest));
                    interest
                } else {
                    harvest_logs.push(format!("Operating Expense: {} - No interest (debt: $0)", expense_card.title));
                    0
                }
            },
            _ => {
                harvest_logs.push(format!("Operating Expense: {} - None", expense_card.title));
                0 // Default to 0 for unhandled effect types
            }
        };
        
        // 2. Calculate harvest income
        let (income, resolve_logs) = match harvest_type {
            HarvestType::Corn | HarvestType::Wheat => {
                let (income_result, logs) = self.resolve_grain_harvest(player, AssetType::Grain, harvest_type, expense)?;
                (income_result, logs)
            }
            HarvestType::Apple | HarvestType::Cherry => {
                let (income_result, logs) = self.resolve_fruit_harvest(player, harvest_type, expense)?;
                (income_result, logs)
            }
            HarvestType::Livestock => {
                let (income_result, logs) = self.resolve_livestock_harvest(player, harvest_type, expense)?;
                (income_result, logs)
            }
            HarvestType::HayCutting1 | HarvestType::HayCutting2 | 
            HarvestType::HayCutting3 | HarvestType::HayCutting4 => {
                let (income_result, logs) = self.resolve_hay_harvest(player, harvest_type, expense)?;
                (income_result, logs)
            }
            _ => (0, vec![]) // No income, no logs for HarvestType::None
        };
        
        harvest_logs.extend(resolve_logs); // Add logs from the specific resolve function

        // Reset crop multipliers after the harvest is completed
        player.reset_crop_multipliers();

        // Discard the expense card
        self.operating_cost_deck.discard_pile.push(expense_card);

        Ok((income - expense, expense, harvest_logs))
    }

    // Modified helper to return logs
    fn resolve_harvest_helper(&mut self, player: &Player, asset: AssetType, yield_table: &[(i32, i32); 6], harvest_type: &HarvestType, expense: i32) -> Result<(i32, Vec<String>), String> {
        let mut logs = Vec::new();
        let quantity = player.assets.get(&asset).map(|a| a.quantity).unwrap_or(0);
        if quantity == 0 {
            logs.push(format!("No {:?} to harvest.", asset));
            return Ok((0, logs)); 
        }

        let units_per_block = match asset {
            AssetType::Hay | AssetType::Grain => 10,
            AssetType::Fruit => 5,
            AssetType::Cows => 10,
            _ => return Err("Unsupported asset type for harvest calculation".to_string()),
        };

        let blocks = quantity / units_per_block;
        if blocks == 0 {
            logs.push(format!("Not enough {:?} for harvest (need {}).", asset, units_per_block));
            return Ok((0, logs)); 
        }

        let roll = rand::thread_rng().gen_range(0..6u8);
        let (base, increment) = yield_table[roll as usize];
        let blocks_minus_one = blocks.saturating_sub(1);
        let increment_total = increment * blocks_minus_one;
        let initial_income = base + increment_total;
        
        let mut final_income = initial_income as f32;

        // Format the harvest name based on type
        let harvest_name = match harvest_type {
            HarvestType::HayCutting1 => "Hay: First Cutting",
            HarvestType::HayCutting2 => "Hay: Second Cutting",
            HarvestType::HayCutting3 => "Hay: Third Cutting",
            HarvestType::HayCutting4 => "Hay: Fourth Cutting",
            HarvestType::Wheat => "Wheat",
            HarvestType::Corn => "Corn",
            HarvestType::Apple => "Apple",
            HarvestType::Cherry => "Cherry",
            HarvestType::Livestock => "Livestock Sales",
            HarvestType::None => "Unknown Harvest",
        };
        
        let mut harvest_msg = format!("{}: Roll {} = ${}/block x {} {}", 
            harvest_name,
            roll + 1,
            base,
            quantity,
            if asset == AssetType::Cows { "cows" } else { "acres" }
        );
        
        // Apply crop multiplier
        let crop_multiplier = player.get_crop_multiplier(&asset);
        if (crop_multiplier - 1.0).abs() > f32::EPSILON {
            final_income *= crop_multiplier;
            harvest_msg.push_str(&format!(" x{:.1} multiplier", crop_multiplier));
        }
        
        // Apply livestock bonus if this is a livestock harvest
        if asset == AssetType::Cows {
            let livestock_multiplier = player.get_livestock_harvest_multiplier();
            if (livestock_multiplier - 1.0).abs() > f32::EPSILON {
                final_income *= livestock_multiplier;
                harvest_msg.push_str(&format!(" x{:.1} livestock", livestock_multiplier));
            }
        }

        let rounded_income = final_income.round() as i32;
        harvest_msg.push_str(&format!(" - ${} operating expense = ${}", expense, rounded_income - expense));
        logs.push(harvest_msg);

        Ok((rounded_income - expense, logs))
    }

    // Update wrappers to pass harvest_type and expense
    pub fn resolve_hay_harvest(&mut self, player: &Player, harvest_type: &HarvestType, expense: i32) -> Result<(i32, Vec<String>), String> {
        let hay_table = [(400, 400), (600, 600), (1000, 1000), (1500, 1500), (2200, 2200), (3000, 3000)];
        self.resolve_harvest_helper(player, AssetType::Hay, &hay_table, harvest_type, expense)
    }

    pub fn resolve_fruit_harvest(&mut self, player: &Player, harvest_type: &HarvestType, expense: i32) -> Result<(i32, Vec<String>), String> {
        let fruit_table = [(2000, 2000), (3500, 3500), (6000, 6000), (9000, 9000), (13000, 13000), (17500, 17500)];
        self.resolve_harvest_helper(player, AssetType::Fruit, &fruit_table, harvest_type, expense)
    }

    pub fn resolve_grain_harvest(&mut self, player: &Player, crop: AssetType, harvest_type: &HarvestType, expense: i32) -> Result<(i32, Vec<String>), String> {
        let grain_table = [(800, 800), (1500, 1500), (2500, 2500), (3800, 3800), (5300, 5300), (7000, 7000)];
        self.resolve_harvest_helper(player, crop, &grain_table, harvest_type, expense)
    }

    pub fn resolve_livestock_harvest(&mut self, player: &Player, harvest_type: &HarvestType, expense: i32) -> Result<(i32, Vec<String>), String> {
        let livestock_table = [(1400, 1400), (2000, 2000), (2800, 2800), (3800, 3800), (5000, 5000), (7500, 7500)];
        self.resolve_harvest_helper(player, AssetType::Cows, &livestock_table, harvest_type, expense)
    }
} 