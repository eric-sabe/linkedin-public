use crate::models::{Card, AssetType};
use crate::game::GameEffect;
use crate::cards::card::CardSource;
use CardSource::*;
use crate::models::player::EffectType;

pub fn operating_expense_catalog() -> Vec<Card> {
    vec![
        Card { 
            id: 100, 
            title: "Fertilizer Bill".to_string(), 
            description: "Fertilizer Bill. Pay $100 per acre.".to_string(), 
            description_brief: "Fertilizer Bill. Pay $100 per acre.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Grain, rate: 100 }, 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 101, 
            title: "Fuel Bill".to_string(), 
            description: "Fuel Bill. Pay $1,000.".to_string(), 
            description_brief: "Fuel Bill. Pay $1,000.".to_string(),
            effect: GameEffect::Expense(1000), 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 102, 
            title: "Electric Bill for Irrigation".to_string(), 
            description: "Electric Bill for Irrigation. Pay $500.".to_string(), 
            description_brief: "Electric Bill for Irrigation. Pay $500.".to_string(),
            effect: GameEffect::Expense(500), 
            default_quantity: 1, 
            source: BaseGame 
        },
        Card { 
            id: 103, 
            title: "Custom Hire - No Tractor".to_string(), 
            description: "Pay $2,000 if you do not own a Tractor.".to_string(), 
            description_brief: "Pay $2,000 if you do not own a Tractor.".to_string(),
            effect: GameEffect::PayIfNoAssetDistribute { required_asset: AssetType::Tractor, amount: 2000 }, 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 104, 
            title: "Custom Hire - No Harvester".to_string(), 
            description: "Pay $2,000 if you do not own a Harvester.".to_string(), 
            description_brief: "Pay $2,000 if you do not own a Harvester.".to_string(),
            effect: GameEffect::PayIfNoAssetDistribute { required_asset: AssetType::Harvester, amount: 2000 }, 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 105, 
            title: "Parts Bill".to_string(), 
            description: "Parts Bill. Pay $500.".to_string(), 
            description_brief: "Parts Bill. Pay $500.".to_string(),
            effect: GameEffect::Expense(500), 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 106, 
            title: "Wire Worm in Grain".to_string(), 
            description: "Wire Worm in Grain. Pay $100 per Grain acre to fumigate.".to_string(), 
            description_brief: "Wire Worm in Grain. Pay $100 per Grain acre to fumigate.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Grain, rate: 100 }, 
            default_quantity: 1, 
            source: BaseGame 
        },
        Card { 
            id: 107, 
            title: "Equipment Breakdown".to_string(), 
            description: "Equipment Breakdown. Pay $500.".to_string(), 
            description_brief: "Equipment Breakdown. Pay $500.".to_string(),
            effect: GameEffect::Expense(500), 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 108, 
            title: "Feed Bill".to_string(), 
            description: "Feed Bill. Pay $100 per cow.".to_string(), 
            description_brief: "Feed Bill. Pay $100 per cow.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Cows, rate: 100 }, 
            default_quantity: 1, 
            source: BaseGame 
        },
        Card { 
            id: 109, 
            title: "Farmowner's Insurance".to_string(), 
            description: "Farmowner's Insurance. Pay $1,500.".to_string(), 
            description_brief: "Farmowner's Insurance. Pay $1,500.".to_string(),
            effect: GameEffect::Expense(1500), 
            default_quantity: 1, 
            source: BaseGame 
        },
        Card { 
            id: 110, 
            title: "Seed Bill".to_string(), 
            description: "Seed Bill. Pay $3,000.".to_string(), 
            description_brief: "Seed Bill. Pay $3,000.".to_string(),
            effect: GameEffect::Expense(3000), 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 111, 
            title: "Farm Taxes".to_string(), 
            description: "Farm Taxes. Pay $1,500.".to_string(), 
            description_brief: "Farm Taxes. Pay $1,500.".to_string(),
            effect: GameEffect::Expense(1500), 
            default_quantity: 1, 
            source: BaseGame 
        },
        Card { 
            id: 112, 
            title: "Interest on Bank Notes".to_string(), 
            description: "Pay 10% on Bank Notes on hand.".to_string(), 
            description_brief: "Pay 10% on Bank Notes on hand.".to_string(),
            effect: GameEffect::PayInterest, 
            default_quantity: 2, 
            source: BaseGame 
        },
        Card { 
            id: 113, 
            title: "Veterinary Bill (Cows)".to_string(), 
            description: "Veterinary Bill. Pay $500 if you own cows.".to_string(), 
            description_brief: "Veterinary Bill. Pay $500 if you own cows.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Cows, rate: 500 }, 
            default_quantity: 1, 
            source: BaseGame 
        },
        Card { 
            id: 114, 
            title: "Equipment in Shop".to_string(), 
            description: "Equipment in the shop. Pay $1,000 for the delay.".to_string(), 
            description_brief: "Equipment in the shop. Pay $1,000 for the delay.".to_string(),
            effect: GameEffect::Expense(1000), 
            default_quantity: 2, 
            source: BaseGame 
        }
    ]
}

// Correct per game board
pub fn farmers_fate_catalog() -> Vec<Card> {
    vec![
        Card {
            id: 200,
            title: "Calves Market Jump".to_string(),
            description: "Held some of your calves and the market jumped. Collect $2,000 if you have cows.".to_string(),
            description_brief: "Collect $2,000 if you have cows.".to_string(),
            effect: GameEffect::IncomeIfHas { asset: AssetType::Cows, amount: 2000 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 201,
            title: "Federal Crop Disaster".to_string(),
            description: "Federal Crop Disaster payment saves your bacon. Collect $100 per Grain acre.".to_string(),
            description_brief: "Taxpayers bailed you out. Collect $100 per Grain acre.".to_string(),
            effect: GameEffect::IncomePerAsset { asset: AssetType::Grain, rate: 100 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 202,
            title: "Bad at Taxes".to_string(),
            description: "IRS garnishes your income after finding errors on your tax return. Draw an Operating Expense card during Harvest but do not roll for Harvest costs.".to_string(),
            description_brief: "No income for you this year - only Operating Expenses!".to_string(),
            effect: GameEffect::DrawOperatingExpenseNoHarvest,
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 205,
            title: "Drought Year".to_string(),
            description: "Drought year! Go to the 2nd week of January. Do not collect your $5,000 year's wages.".to_string(),
            description_brief: "Drought year! Skip to 2nd week of January. Do not collect $5,000.".to_string(),
            effect: GameEffect::SkipYear,
            default_quantity: 2,
            source: BaseGame
        },
        Card {
            id: 206,
            title: "Truckers Strike".to_string(),
            description: "Truckers strike delays Fruit in transport, lots of spoilage. Pay $1,000 per Fruit acre.".to_string(),
            description_brief: "Truckers strike: Transport delays cause spoilage. Pay $1,000 per Fruit acre.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Fruit, rate: 1000 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 207,
            title: "Uncle Bert's Legacy".to_string(),
            description: "Uncle Bert dies and leaves you 10 acres of Hay, if you can raise the $10,000 cash to pay Inheritance Tax and small remaining mortgage.".to_string(),
            description_brief: "Uncle Bert dies: Inherit 10 acres of Hay for $10,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Hay, quantity: 10, cost: 10000 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 208,
            title: "Premium Hay Sale".to_string(),
            description: "Rich folks from the city bought the neighboring farm and pay you a premium for your best hay to feed their show horses. Collect $100 per Hay Acre.".to_string(),
            description_brief: "Premium Hay Sale: Collect $100 per Hay Acre.".to_string(),
            effect: GameEffect::IncomePerAsset { asset: AssetType::Hay, rate: 100 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 209,
            title: "Weed Infestation".to_string(),
            description: "Windy spring, didn't get your wheat sprayed. Weeds cut your wheat crop in half. Hold this card through Wheat Harvest for this year.".to_string(),
            description_brief: "Weeds cut your wheat crop in half.".to_string(),
            effect: GameEffect::OneTimeHarvestMultiplier { asset: AssetType::Grain, multiplier: 0.5 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 210,
            title: "Cherry Market Crash".to_string(),
            description: "Some TV talk show host does a show on the dangers of farm control spray use on your cherries. Even though the pseudo-science behind his ill-informed rant made you angrier than a supermarket-rabid, the national cherry market crashes. Cut your cherry crop in half if you haven't already harvested this year.".to_string(),
            description_brief: "TV talking head ruins cherry market. Cut your cherry crop in half.".to_string(),
            effect: GameEffect::OneTimeHarvestMultiplier { asset: AssetType::Fruit, multiplier: 0.5 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 211,
            title: "Income Taxes Due".to_string(),
            description: "Income taxes due. Pay $7,000.".to_string(),
            description_brief: "Income taxes due. Pay $7,000.".to_string(),
            effect: GameEffect::Expense(7000),
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 212,
            title: "Mt. St. Helens Disaster".to_string(),
            description: "Mt. St. Helens Blows. You are luckily out of the Ash Path.  Your ash-free hay jumps in price! Collect $500 per Hay acre. Other players must roll to see if they escaped. Odd-escaped, Even-hit! Ash hit players Pay $100 per acre (all crops) to clean up mess.".to_string(),
            description_brief: "Volcano! You are safe and collect $500 per Hay acre. Other players roll to escape or pay.".to_string(),
            effect: GameEffect::MtStHelensDisaster,
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 213,
            title: "Cut Worms".to_string(),
            description: "Cut worms eat sprouting Fruit buds. EPA bans control spray. Pay $300 per Fruit acre.".to_string(),
            description_brief: "Worms in the fruit. EPA bans spray. Pay $300 per Fruit acre.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Fruit, rate: 800 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 214,
            title: "Prime Rate Hike".to_string(),
            description: "Banks raise Prime Rate. Pay 10% of outstanding loan balance as additional interest.".to_string(),
            description_brief: "Prime Rate Hike. Pay 10% of outstanding loan balance.".to_string(),
            effect: GameEffect::PayInterest,
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 215,
            title: "PCB Contamination".to_string(),
            description: "A leaking electrical motor at Feed Mill contaminated your load of feed with PCB. State Ag Inspector requires you slaughter all cows (not cows on lease range land) with no reimbursement.".to_string(),
            description_brief: "Leaky motor contaiminates feed. Slaughter cows without compensation.".to_string(),
            effect: GameEffect::SlaughterCowsWithoutCompensation,
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 216,
            title: "Cattle Management Bonus".to_string(),
            description: "Sharp management, testing and your computer record system cause your cattle yearling weights to soar. Receive a 50% bonus after you roll for your Livestock Harvest check each of the next two years.".to_string(),
            description_brief: "You're a cattle management genius. 50% bonus for 2 years.".to_string(),
            effect: GameEffect::AddPersistentEffect {
                effect_type: EffectType::LivestockHarvestBonus(1.5),
                years: 2
            },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 217,
            title: "Tractor Hire Bill - $3,000".to_string(),
            description: "Custom hire bill due. If you have no Tractor Pay $3,000.".to_string(),
            description_brief: "Pay $3,000 to hire a tractor.".to_string(),
            effect: GameEffect::PayIfNoAssetDistribute { required_asset: AssetType::Tractor, amount: 3000 },
            default_quantity: 2,
            source: BaseGame
        },
        Card {
            id: 219,
            title: "Russian Wheat Sale".to_string(),
            description: "Russian sale boosts wheat prices. Collect $2,000.".to_string(),
            description_brief: "Putin buys your wheat. Collect $2,000.".to_string(),
            effect: GameEffect::Income(2000),
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 220,
            title: "Apple Maggot Fly".to_string(),
            description: "The Apple Maggot Fly, cousin of the dreaded Medfly, is found in an insect trap in your Fruit. Your Fruit is quarantined, but you get a lucrative export contract. Pay $500 per Fruit acre.".to_string(),
            description_brief: "Stupid Apple Maggot fly. Pay $500 per Fruit acre.".to_string(),
            effect: GameEffect::ExpensePerAsset { asset: AssetType::Fruit, rate: 500 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 221,
            title: "Grain Embargo".to_string(),
            description: "The President slaps on a Grain Embargo while you're waiting for the custom harvester to show up. Instant market collapse. Pay $2,500 if you don't own your own Harvester.".to_string(),
            description_brief: "Trump and his grain embargos! Pay $2,500 if you don't own your own Harvester.".to_string(),
            effect: GameEffect::PayIfNoAssetDistribute { required_asset: AssetType::Harvester, amount: 2500 },
            default_quantity: 1,
            source: BaseGame
        },
        Card {
            id: 222,
            title: "Marketing Co-op Success".to_string(),
            description: "Marketing Co-op holds out for higher price. Processor gives in! Collect $1,000.".to_string(),
            description_brief: "Marketing Co-op negotiates $1,000 bonus.".to_string(),
            effect: GameEffect::Income(1000),
            default_quantity: 1,
            source: BaseGame
        }
    ]
}

// Correct per game board
pub fn option_to_buy_catalog() -> Vec<Card> {
    vec![
        Card {
            id: 300,
            title: "Livestock Auction".to_string(),
            description: "Livestock auction 10 pregnant cows at $500 each Total $5,000".to_string(),
            description_brief: "Buy 10 cows for $5,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Cows, quantity: 10, cost: 5000 },
            default_quantity: 6,
            source: BaseGame
        },
        Card {
            id: 301,
            title: "Buy Grain Land".to_string(),
            description: "Neighbor sells out 10 acres of Grain at $2,000 per acre Total $20,000".to_string(),
            description_brief: "Buy 10 acres of Grain at $2,000 per acre for $20,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Grain, quantity: 10, cost: 20000 },
            default_quantity: 5,
            source: BaseGame
        },
        Card {
            id: 302,
            title: "Buy Fruit Land".to_string(),
            description: "Neighbor goes broke 5 acres of Fruit at $5,000 per acre Total $25,000".to_string(),
            description_brief: "Buy 5 acres of Fruit at $5,000 per acre for $25,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Fruit, quantity: 5, cost: 25000 },
            default_quantity: 6,
            source: BaseGame
        },
        Card {
            id: 303,
            title: "Buy Used Tractor".to_string(),
            description: "Equipment sale old but useable tractor Total $10,000".to_string(),
            description_brief: "Buy a used tractor for $10,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Tractor, quantity: 1, cost: 10000 },
            default_quantity: 3,
            source: BaseGame
        },
        Card {
            id: 304,
            title: "Buy Used Harvester".to_string(),
            description: "Equipment sale old but useable harvester Total $10,000".to_string(),
            description_brief: "Buy a used harvester for $10,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Harvester, quantity: 1, cost: 10000 },
            default_quantity: 3,
            source: BaseGame
        },
        Card {
            id: 305,
            title: "Lease Toppenish Ridge".to_string(),
            description: "Lease Toppenish Ridge for lifetime at $25,000 and buy 50 pregnant cows to stock it at $500 each Total $50,000".to_string(),
            description_brief: "Lease Toppenish Ridge and buy 50 cows for $50,000.".to_string(),
            effect: GameEffect::LeaseRidge { 
                name: "Toppenish Ridge".to_string(), 
                cost: 50000,
                cow_count: 50 
            },
            default_quantity: 3,
            source: BaseGame
        },
        Card {
            id: 306,
            title: "Lease Rattlesnake Ridge".to_string(),
            description: "Lease Rattlesnake Ridge for lifetime at $15,000 and buy 30 pregnant cows to stock it at $500 each Total $30,000".to_string(),
            description_brief: "Lease Rattlesnake Ridge and buy 30 cows for $30,000.".to_string(),
            effect: GameEffect::LeaseRidge { 
                name: "Rattlesnake Ridge".to_string(), 
                cost: 30000,
                cow_count: 30 
            },
            default_quantity: 3,
            source: BaseGame
        },
        Card {
            id: 307,
            title: "Lease Ahtanum Ridge".to_string(),
            description: "Lease Ahtanum Ridge for lifetime at $10,000 and buy 20 pregnant cows to stock it at $500 each Total $20,000".to_string(),
            description_brief: "Lease Ahtanum Ridge and buy 20 cows for $20,000.".to_string(),
            effect: GameEffect::LeaseRidge { 
                name: "Ahtanum Ridge".to_string(), 
                cost: 20000,
                cow_count: 20 
            },
            default_quantity: 3,
            source: BaseGame
        },
        Card {
            id: 308,
            title: "Lease Cascade Ridge".to_string(),
            description: "Lease Cascade Range for lifetime at $20,000 and buy 40 pregnant cows to stock it at $500 each Total $40,000".to_string(),
            description_brief: "Lease Cascade Range and buy 40 cows for $40,000.".to_string(),
            effect: GameEffect::LeaseRidge { 
                name: "Cascade Ridge".to_string(), 
                cost: 40000,
                cow_count: 40 
            },
            default_quantity: 3,
            source: BaseGame
        },
        Card {
            id: 309,
            title: "Buy Hay Land".to_string(),
            description: "Neighbor sells out 10 acres of Hay at $2,000 per acre Total $20,000".to_string(),
            description_brief: "Buy 10 acres of Hay at $2,000 per acre for $20,000.".to_string(),
            effect: GameEffect::OptionalBuyAsset { asset: AssetType::Hay, quantity: 10, cost: 20000 },
            default_quantity: 5,
            source: BaseGame
        },
    ]
} 