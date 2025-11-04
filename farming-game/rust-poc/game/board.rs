use crate::models::{GameState, BoardTile, TileType, HarvestType, TileEffect, Player, Ridge};
use crate::game::GameEffect;
use crate::models::asset::AssetType;
use std::collections::HashMap;

impl From<TileEffect> for GameEffect {
    fn from(effect: TileEffect) -> Self {
        match effect {
            TileEffect::None => GameEffect::Special("No effect".to_string()),
            TileEffect::DrawCard(tile_type) => GameEffect::Special(format!("Draw a {:?} card", tile_type)),
            TileEffect::GainCash(amount) => GameEffect::Income(amount),
            TileEffect::PayCash(amount) => GameEffect::Expense(amount),
            TileEffect::SkipYear => GameEffect::SkipYear,
            TileEffect::GoToTile(position) => GameEffect::Special(format!("Move to position {}", position)),
            TileEffect::Special(desc) => GameEffect::Special(desc),
            TileEffect::ExpensePerAsset { asset, rate } => GameEffect::ExpensePerAsset { asset, rate },
            TileEffect::DoubleYieldForCrop(asset) => GameEffect::Special(format!("Double yield for {:?}", asset)),
            TileEffect::PayInterest => GameEffect::PayInterest,
            TileEffect::GoToTileAndGainCash { tile_index: _, amount } => GameEffect::Income(amount),
            TileEffect::GainCashIfAsset { asset: _, amount } => GameEffect::Income(amount),
            TileEffect::HarvestBonusPerAcre { asset, bonus } => GameEffect::Special(format!("Add ${} per acre to {:?} harvest", bonus, asset)),
            TileEffect::MoveAndHarvestIfAsset { asset, destination, bonus, harvest_type } => GameEffect::Special(format!("Move and harvest if asset {:?} to tile {} with bonus ${} and harvest type {:?}", asset, destination, bonus, harvest_type)),
            TileEffect::OneTimeHarvestMultiplier { asset, multiplier } => GameEffect::Special(format!("Market collapse. Cut livestock check in half. Multiplier: {}", multiplier)),
            TileEffect::PayCashIfAsset { asset, amount } => GameEffect::Expense(amount),
        }
    }
}

fn generate_brief_description(description: &str) -> String {
    // If description is already brief (less than 50 chars), return it as is
    if description.len() < 50 {
        return description.to_string();
    }

    // Extract key information based on common patterns
    if description.contains("COLLECT") {
        if let Some(amount) = description.find('$') {
            let end = description[amount..].find('.').unwrap_or(description.len() - amount);
            return description[..amount+end+1].to_string();
        }
    }
    
    if description.contains("PAY") {
        if let Some(amount) = description.find('$') {
            let end = description[amount..].find('.').unwrap_or(description.len() - amount);
            return description[..amount+end+1].to_string();
        }
    }

    // For movement instructions, keep just the destination
    if description.contains("Go to") {
        if let Some(go_to) = description.find("Go to") {
            let end = description[go_to..].find('.').unwrap_or(description.len() - go_to);
            return description[go_to..go_to+end+1].to_string();
        }
    }

    // For harvest effects, keep the key effect
    if description.contains("harvest") {
        if description.contains("Double") {
            return "Double harvest this year.".to_string();
        }
        if description.contains("Cut") && description.contains("half") {
            return "Cut harvest in half.".to_string();
        }
    }

    // For card draws, keep it simple
    if description.contains("Draw") {
        if description.contains("O.T.B.") {
            return "Draw O.T.B.".to_string();
        }
        if description.contains("Farmer's Fate") {
            return "Draw Farmer's Fate.".to_string();
        }
    }

    // For complex descriptions, try to extract the main action
    let first_sentence_end = description.find('.').unwrap_or(description.len());
    description[..first_sentence_end+1].to_string()
}

pub fn create_full_board() -> Vec<BoardTile> {
    vec![
        // January Tiles
        BoardTile {
            index: 0,
            name: "Christmas Vacation".to_string(),
            tile_type: TileType::SpecialEvent,
            harvest_type: HarvestType::None,
            effect: TileEffect::GainCash(1000),
            description: Some("COLLECT $1000 Christmas bonus!".to_string()),
            description_brief: Some("COLLECT $1000.".to_string()),
        },
        BoardTile {
            index: 1,
            name: "January Week 1".to_string(),
            tile_type: TileType::PayInterest,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayInterest,
            description: Some("PAY 10% interest on Bank Notes".to_string()),
            description_brief: Some("PAY 10% interest.".to_string()),
        },
        BoardTile {
            index: 2,
            name: "January Week 2".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::None,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Hibernate. Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },
        BoardTile {
            index: 3,
            name: "January Week 3".to_string(),
            tile_type: TileType::PayIfAssetOwned,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCashIfAsset { asset: AssetType::Cows, amount: 500 },
            description: Some("Bitter cold spell. PAY $500 if you own cows.".to_string()),
            description_brief: Some("PAY $500 if you own cows.".to_string()),
        },
        BoardTile {
            index: 4,
            name: "January Week 4".to_string(),
            tile_type: TileType::DoubleYieldForCrop,
            harvest_type: HarvestType::None,
            effect: TileEffect::DoubleYieldForCrop(AssetType::Hay),
            description: Some("Beautiful Days! Double all your Hay harvests this year.".to_string()),
            description_brief: Some("Double all your Hay harvests this year.".to_string()),
        },

        // February Tiles
        BoardTile {
            index: 5,
            name: "February Week 1".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::None,
            effect: TileEffect::GainCash(1000),
            description: Some("Warm snap, you're in the field 2 weeks early. Collect $1000.".to_string()),
            description_brief: Some("Collect $1000.".to_string()),
        },
        BoardTile {
            index: 6,
            name: "February Week 2".to_string(),
            tile_type: TileType::FarmerFate,
            harvest_type: HarvestType::None,
            effect: TileEffect::DrawCard(TileType::FarmerFate),
            description: Some("Stuck in a muddy canal. Draw Farmer's Fate.".to_string()),
            description_brief: Some("Draw Farmer's Fate.".to_string()),
        },
        BoardTile {
            index: 7,
            name: "February Week 3".to_string(),
            tile_type: TileType::JumpToTile,
            harvest_type: HarvestType::None,
            effect: TileEffect::GoToTile(14),
            description: Some("Ground thaws. Start planting early crops. Go to Spring Planting.".to_string()),
            description_brief: Some("Go to Spring Planting.".to_string()),
        },
        BoardTile {
            index: 8,
            name: "February Week 4".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::None,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Rainy day. Draw O.T.B.".to_string()),
            description_brief: Some("Rainy day. Draw O.T.B.".to_string()),
        },

        // March Tiles
        BoardTile {
            index: 9,
            name: "March Week 1".to_string(),
            tile_type: TileType::PayFees,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCash(2000),
            description: Some("Becomes obvious your wheat has winter killed. PAY $2000 to replant.".to_string()),
            description_brief: Some("PAY $2000 to replant.".to_string()),
        },
        BoardTile {
            index: 10,
            name: "March Week 2".to_string(),
            tile_type: TileType::PayFees,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCash(500),
            description: Some("Start plowing late. PAY $500.".to_string()),
            description_brief: Some("PAY $500.".to_string()),
        },
        BoardTile {
            index: 11,
            name: "Hurt Back".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::None,
            effect: TileEffect::SkipYear,
            description: Some("Hurt your back. Skip a year.".to_string()),
            description_brief: Some("Skip a year.".to_string()),
        },
        BoardTile {
            index: 12,
            name: "March Week 4".to_string(),
            tile_type: TileType::PayIfAssetOwned,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCashIfAsset { asset: AssetType::Fruit, amount: 2000 },
            description: Some("Frost forces you to heat fruit. PAY $2000 if you own fruit.".to_string()),
            description_brief: Some("PAY $2000 if you own fruit.".to_string()),
        },

        // April Tiles
        BoardTile {
            index: 13,
            name: "April Week 1".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::None,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Done plowing. Take a day off. Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },

        // Spring Planting
        BoardTile {
            index: 14,
            name: "Spring Planting".to_string(),
            tile_type: TileType::DoubleYieldForCrop,
            harvest_type: HarvestType::None,
            effect: TileEffect::DoubleYieldForCrop(AssetType::Grain),
            description: Some("Plant corn on time. Double corn yield this year.".to_string()),
            description_brief: Some("Plant corn on time. Double corn yield this year.".to_string()),
        },

        // More April Tiles
        BoardTile {
            index: 15,
            name: "April Week 2".to_string(),
            tile_type: TileType::PayFees,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCash(500),
            description: Some("More rain. Field work shut down. PAY $500.".to_string()),
            description_brief: Some("PAY $500.".to_string()),
        },
        BoardTile {
            index: 16,
            name: "April Week 3".to_string(),
            tile_type: TileType::PayFees,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCash(1000),
            description: Some("Equipment breakdown. PAY $1000.".to_string()),
            description_brief: Some("PAY $1000.".to_string()),
        },

        // May Tiles
        BoardTile {
            index: 17,
            name: "May Week 1".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::None,
            effect: TileEffect::GainCash(500),
            description: Some("The whole valley is green. COLLECT $500.".to_string()),
            description_brief: Some("COLLECT $500.".to_string()),
        },
        BoardTile {
            index: 18,
            name: "May Week 2".to_string(),
            tile_type: TileType::PayFees,
            harvest_type: HarvestType::None,
            effect: TileEffect::PayCash(500),
            description: Some("Windstorm makes you replant corn. PAY $500.".to_string()),
            description_brief: Some("PAY $500.".to_string()),
        },
        BoardTile {
            index: 19,
            name: "May Week 3".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::HayCutting1,
            effect: TileEffect::GainCash(1000),
            description: Some("Cut your hay just right. COLLECT $1000 bonus.".to_string()),
            description_brief: Some("COLLECT $1000 bonus.".to_string()),
        },
        BoardTile {
            index: 20,
            name: "May Week 4".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::HayCutting1,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Memorial Day weekend. Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },

        // June Tiles
        BoardTile {
            index: 21,
            name: "June Week 1".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::HayCutting1,
            effect: TileEffect::OneTimeHarvestMultiplier { asset: AssetType::Hay, multiplier: 0.5 },
            description: Some("Rain storm ruins unbaled hay. Cut your harvest check in half.".to_string()),
            description_brief: Some("Cut your harvest check in half.".to_string()),
        },
        BoardTile {
            index: 22,
            name: "June Week 2".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::HayCutting1,
            effect: TileEffect::GainCash(500),
            description: Some("Good growing weather. COLLECT $500 bonus.".to_string()),
            description_brief: Some("COLLECT $500 bonus.".to_string()),
        },
        BoardTile {
            index: 23,
            name: "June Week 3".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::Cherry,
            effect: TileEffect::OneTimeHarvestMultiplier { asset: AssetType::Fruit, multiplier: 0.5 },
            description: Some("Rain splits your cherries. Cut your harvest check in half.".to_string()),
            description_brief: Some("Cut your harvest check in half.".to_string()),
        },
        BoardTile {
            index: 24,
            name: "June Week 4".to_string(),
            tile_type: TileType::FarmerFate,
            harvest_type: HarvestType::Cherry,
            effect: TileEffect::DrawCard(TileType::FarmerFate),
            description: Some("Dust storm. Draw Farmer's Fate.".to_string()),
            description_brief: Some("Draw Farmer's Fate.".to_string()),
        },

        BoardTile {
            index: 25,
            name: "Independence Day Bash".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::Cherry,
            effect: TileEffect::None,
            description: Some("Independence Day Bash".to_string()),
            description_brief: Some("Independence Day Bash".to_string()),
        },

        // July Tiles
        BoardTile {
            index: 26,
            name: "July Week 1".to_string(),
            tile_type: TileType::DoubleYieldForCrop,
            harvest_type: HarvestType::HayCutting2,
            effect: TileEffect::DoubleYieldForCrop(AssetType::Hay),
            description: Some("Good weather for your second cutting of hay. Double Hay harvest check.".to_string()),
            description_brief: Some("Double Hay harvest check.".to_string()),
        },
        BoardTile {
            index: 27,
            name: "July Week 2".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::HayCutting2,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Hot! Wish you were in the mountains! Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },
        BoardTile {
            index: 28,
            name: "July Week 3".to_string(),
            tile_type: TileType::JumpToTile,
            harvest_type: HarvestType::HayCutting2,
            effect: TileEffect::GoToTile(37),
            description: Some("It's a cooker! 114° in the shade. Wipe your brow and go to Harvest Moon after getting Hay check.".to_string()),
            description_brief: Some("Go to Harvest Moon after getting Hay check.".to_string()),
        },
        BoardTile {
            index: 29,
            name: "July Week 4".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::Wheat,
            effect: TileEffect::HarvestBonusPerAcre { asset: AssetType::Grain, bonus: 50 },
            description: Some("85°, wheat heads filling out beautifully. Add $50 per acre to your harvest check.".to_string()),
            description_brief: Some("Add $50 per acre to your harvest check.".to_string()),
        },

        // August Tiles
        BoardTile {
            index: 30,
            name: "August Week 1".to_string(),
            tile_type: TileType::JumpToTile,
            harvest_type: HarvestType::Wheat,
            effect: TileEffect::GoToTileAndGainCash { tile_index: 8, amount: 5000 },
            description: Some("You're right on time and working like a pro. Go to the fourth week of February. COLLECT your year's wage of $5000.".to_string()),
            description_brief: Some("COLLECT your year's wage of $5000.".to_string()),
        },
        BoardTile {
            index: 31,
            name: "August Week 2".to_string(),
            tile_type: TileType::PayIfAssetOwned,
            harvest_type: HarvestType::Wheat,
            effect: TileEffect::GainCashIfAsset { asset: AssetType::Harvester, amount: 1000 },
            description: Some("Storm clouds brewing. COLLECT $1000 if you have a Harvester.".to_string()),
            description_brief: Some("COLLECT $1000 if you have a Harvester.".to_string()),
        },
        BoardTile {
            index: 32,
            name: "August Week 3".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::Wheat,
            effect: TileEffect::GainCash(500),
            description: Some("Finish wheat harvesting with no breakdowns. COLLECT $500.".to_string()),
            description_brief: Some("COLLECT $500.".to_string()),
        },
        BoardTile {
            index: 33,
            name: "August Week 4".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::Wheat,
            effect: TileEffect::HarvestBonusPerAcre { asset: AssetType::Grain, bonus: -50 },
            description: Some("Rain sprouts unharvested wheat. Cut price $50 per acre on harvest check.".to_string()),
            description_brief: Some("Cut price $50 per acre on harvest check.".to_string()),
        },

        // September Tiles
        BoardTile {
            index: 34,
            name: "September Week 1".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::HayCutting3,
            effect: TileEffect::MoveAndHarvestIfAsset {
                asset: AssetType::Tractor,
                destination: 45,  // November Week 3
                bonus: 1000,
                harvest_type: HarvestType::Apple,
            },
            description: Some("Tractor owners: bale Hay, then go to third week of November. COLLECT $1000 there, then harvest your fruit.".to_string()),
            description_brief: Some("Tractor owners: bale Hay, then go to third week of November. COLLECT $1000 there, then harvest your fruit.".to_string()),
        },
        BoardTile {
            index: 35,
            name: "September Week 2".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::HayCutting3,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Sunny skies at the County Fair. Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },
        BoardTile {
            index: 36,
            name: "September Week 3".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::Livestock,
            effect: TileEffect::OneTimeHarvestMultiplier { asset: AssetType::Cows, multiplier: 0.5 },
            description: Some("Market collapse. Cut livestock check in half.".to_string()),
            description_brief: Some("Cut livestock check in half.".to_string()),
        },
        BoardTile {
            index: 37,
            name: "Harvest Moon".to_string(),
            tile_type: TileType::Special,
            harvest_type: HarvestType::Livestock,
            effect: TileEffect::GainCash(500),
            description: Some("Harvest Moon smiles on you. Collect $500.".to_string()),
            description_brief: Some("Collect $500.".to_string()),
        },
        BoardTile {
            index: 38,
            name: "September Week 4".to_string(),
            tile_type: TileType::PayIfAssetOwned,
            harvest_type: HarvestType::Livestock,
            effect: TileEffect::PayCashIfAsset { asset: AssetType::Fruit, amount: 2000 },
            description: Some("Codling Moth damage to apples lowers fruit grade. PAY $2000 if you own fruit.".to_string()),
            description_brief: Some("PAY $2000 if you own fruit.".to_string()),
        },

        // October Tiles
        BoardTile {
            index: 39,
            name: "October Week 1".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::Livestock,
            effect: TileEffect::GainCash(500),
            description: Some("Indian Summer. Collect $500.".to_string()),
            description_brief: Some("COLLECT $500.".to_string()),
        },
        BoardTile {
            index: 40,
            name: "October Week 2".to_string(),
            tile_type: TileType::FarmerFate,
            harvest_type: HarvestType::HayCutting4,
            effect: TileEffect::DrawCard(TileType::FarmerFate),
            description: Some("Good Pheasant Hunting. Draw Farmer's Fate.".to_string()),
            description_brief: Some("Draw Farmer's Fate.".to_string()),
        },
        BoardTile {
            index: 41,
            name: "October Week 3".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::HayCutting4,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Park your baler for the Winter. Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },
        BoardTile {
            index: 42,
            name: "October Week 4".to_string(),
            tile_type: TileType::FarmerFate,
            harvest_type: HarvestType::Apple,
            effect: TileEffect::DrawCard(TileType::FarmerFate),
            description: Some("Annual Deer Hunt. Draw Farmer's Fate.".to_string()),
            description_brief: Some("Draw Farmer's Fate.".to_string()),
        },

        // November Tiles
        BoardTile {
            index: 43,
            name: "November Week 1".to_string(),
            tile_type: TileType::OptionToBuy,
            harvest_type: HarvestType::Apple,
            effect: TileEffect::DrawCard(TileType::OptionToBuy),
            description: Some("Irrigation Season over. Draw O.T.B.".to_string()),
            description_brief: Some("Draw O.T.B.".to_string()),
        },
        BoardTile {
            index: 44,
            name: "November Week 2".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::Apple,
            effect: TileEffect::GainCash(500),
            description: Some("Good weather, harvest winding up. COLLECT $500.".to_string()),
            description_brief: Some("COLLECT $500.".to_string()),
        },
        BoardTile {
            index: 45,
            name: "November Week 3".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::Corn,
            effect: TileEffect::GainCash(1000),
            description: Some("Good weather holding, COLLECT $1000.".to_string()),
            description_brief: Some("COLLECT $1000.".to_string()),
        },
        BoardTile {
            index: 46,
            name: "November Week 4".to_string(),
            tile_type: TileType::PayIfAssetOwned,
            harvest_type: HarvestType::Corn,
            effect: TileEffect::PayCashIfAsset { asset: AssetType::Fruit, amount: 1000 },
            description: Some("Early freeze kills fruit buds. PAY $1000 if you have Fruit.".to_string()),
            description_brief: Some("PAY $1000 if you have Fruit.".to_string()),
        },

        // December Tiles
        BoardTile {
            index: 47,
            name: "December Week 1".to_string(),
            tile_type: TileType::CollectBonus,
            harvest_type: HarvestType::Corn,
            effect: TileEffect::GainCash(500),
            description: Some("Cold and dry, perfect Field Corn Harvesting. COLLECT $500.".to_string()),
            description_brief: Some("COLLECT $500.".to_string()),
        },
        BoardTile {
            index: 48,
            name: "December Week 2".to_string(),
            tile_type: TileType::FarmerFate,
            harvest_type: HarvestType::Corn,
            effect: TileEffect::DrawCard(TileType::FarmerFate),
            description: Some("First Snow. Draw Farmer's Fate.".to_string()),
            description_brief: Some("Draw Farmer's Fate.".to_string()),
        },
    ]
}

impl GameState {
    pub fn apply_harvest_effect(&mut self, _player_id: usize, tile: &BoardTile) -> Result<(), String> {
        match tile.harvest_type {
            HarvestType::None => Ok(()),
            HarvestType::Corn => {
                // Apply corn harvest
                Ok(())
            }
            HarvestType::Apple => {
                // Apply apple harvest
                Ok(())
            }
            HarvestType::Cherry => {
                // Apply cherry harvest
                Ok(())
            }
            HarvestType::Livestock => {
                // Apply livestock harvest
                Ok(())
            }
            HarvestType::HayCutting1 | HarvestType::HayCutting2 | 
            HarvestType::HayCutting3 | HarvestType::HayCutting4 => {
                // Apply hay harvest
                Ok(())
            }
            HarvestType::Wheat => {
                // Apply wheat harvest
                Ok(())
            }
        }
    }
}

impl Ridge {
    pub fn get_leasee_player<'a>(&self, players: &'a HashMap<usize, Player>) -> Option<&'a Player> {
        if let Some(leasee_id) = self.leased_by {
            players.get(&leasee_id)
        } else {
            None
        }
    }
}

pub fn tile_effect_to_game_effect(effect: &TileEffect) -> GameEffect {
    match effect {
        TileEffect::None => GameEffect::Special("No effect".to_string()),
        TileEffect::DrawCard(card_type) => GameEffect::Special(format!("Draw a {:?} card", card_type)),
        TileEffect::GainCash(amount) => GameEffect::Income(*amount),
        TileEffect::PayCash(amount) => GameEffect::Expense(*amount),
        TileEffect::SkipYear => GameEffect::SkipYear,
        TileEffect::GoToTile(position) => GameEffect::Special(format!("Move to position {}", position)),
        TileEffect::Special(desc) => GameEffect::Special(desc.clone()),
        TileEffect::ExpensePerAsset { asset: _asset, rate } => GameEffect::ExpensePerAsset { asset: *_asset, rate: *rate },
        TileEffect::DoubleYieldForCrop(asset) => GameEffect::Special(format!("Double yield for {:?}", asset)),
        TileEffect::PayInterest => GameEffect::PayInterest,
        TileEffect::GoToTileAndGainCash { tile_index, amount } => {
            GameEffect::Special(format!("Move to tile {} and gain ${}", tile_index, amount))
        },
        TileEffect::GainCashIfAsset { asset, amount } => {
            GameEffect::Special(format!("Gain ${} if you have {:?}", amount, asset))
        },
        TileEffect::PayCashIfAsset { asset: _asset, amount } => {
            GameEffect::Special(format!("Pay ${} if you have {:?}", amount, _asset))
        },
        TileEffect::HarvestBonusPerAcre { asset, bonus } => {
            GameEffect::Special(format!("Gain ${} per {:?} acre", bonus, asset))
        },
        TileEffect::MoveAndHarvestIfAsset { asset, destination, bonus, harvest_type: _harvest_type } => {
            GameEffect::Special(format!("Move to tile {} and harvest {:?} with bonus {}", 
                destination, asset, bonus))
        },
        TileEffect::OneTimeHarvestMultiplier { asset: _asset, multiplier } => GameEffect::Special(format!("Market collapse. Cut livestock check in half ({})", multiplier)),
    }
} 