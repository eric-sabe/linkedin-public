#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AssetType {
    Grain,
    Hay,
    Cows,
    Fruit,
    Tractor,
    Harvester,
} // Correct per game board

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Grain => write!(f, "grain"),
            AssetType::Hay => write!(f, "hay"),
            AssetType::Cows => write!(f, "cows"),
            AssetType::Fruit => write!(f, "Fruit"),
            AssetType::Tractor => write!(f, "tractor"),
            AssetType::Harvester => write!(f, "harvester"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssetRecord {
    pub quantity: i32,
    pub total_cost: i32,
    pub total_income: i32,
} 