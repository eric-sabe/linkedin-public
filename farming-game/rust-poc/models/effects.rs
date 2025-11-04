use crate::models::AssetType;
use crate::models::player::EffectType;

#[derive(Debug, Clone)]
pub enum GameEffect {
    // Card Effects
    CollectFromOthersIfHas { asset: AssetType, amount: i32 },
    IncomeIfHas { asset: AssetType, amount: i32 },
    SuppressHarvestIncome,
    MtStHelensDisaster,
    PayIfNoAssetDistribute { required_asset: AssetType, amount: i32 },
    ExpensePerAsset { asset: AssetType, rate: i32 },
    IncomePerAsset { asset: AssetType, rate: i32 },
    Income(i32),
    Expense(i32),
    AdjustDebt(i32),
    AdjustLand(i32),
    Special(String),
    LeaseRidge { name: String, cost: i32, cow_count: i32 },
    BuyAsset { asset: AssetType, quantity: i32, cost: i32 },
    OptionalBuyAsset { asset: AssetType, quantity: i32, cost: i32 },
    SkipYear,
    AddPersistentEffect { effect_type: EffectType, years: u32 },
    SlaughterCowsWithoutCompensation,
    PayInterest,
    DrawOperatingExpenseNoHarvest,
    OneTimeHarvestMultiplier { asset: AssetType, multiplier: f32 },
    IncomePerLandAcre { rate: i32 },
} 