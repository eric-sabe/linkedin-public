use crate::models::asset::AssetType;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    FarmerFate,
    CropIncome,
    LivestockIncome,
    SpecialEvent,
    PayInterest,
    PayIfAssetOwned,
    DoubleYieldForCrop,
    CollectBonus,
    JumpToTile,
    PayFees,
    OptionToBuy,
    SkipYear,
    Special,
    Blank,
}

// Correct per game board
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HarvestType {
    None,
    Corn,
    Apple,
    Cherry,
    Livestock,
    HayCutting1,
    HayCutting2,
    HayCutting3,
    HayCutting4,
    Wheat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileEffect {
    None,
    DrawCard(TileType),
    GainCash(i32),
    PayCash(i32),
    SkipYear,
    GoToTile(usize),
    Special(String),
    ExpensePerAsset { asset: AssetType, rate: i32 },
    DoubleYieldForCrop(AssetType),
    PayInterest,
    GoToTileAndGainCash { tile_index: usize, amount: i32 },
    GainCashIfAsset { asset: AssetType, amount: i32 },
    PayCashIfAsset { asset: AssetType, amount: i32 },
    HarvestBonusPerAcre { asset: AssetType, bonus: i32 },
    MoveAndHarvestIfAsset { 
        asset: AssetType,
        destination: usize,
        bonus: i32,
        harvest_type: HarvestType,
    },
    OneTimeHarvestMultiplier { asset: AssetType, multiplier: f32 },
}

#[derive(Debug, Clone)]
pub struct BoardTile {
    pub index: usize,
    pub name: String,
    pub tile_type: TileType,
    pub harvest_type: HarvestType,
    pub effect: TileEffect,
    pub description: Option<String>,
    pub description_brief: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Ridge {
    pub name: String,
    pub cow_capacity: u32,
    pub leased_by: Option<usize>, // player ID
} 