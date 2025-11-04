use rand::seq::SliceRandom;
use crate::cards::card::Card;
use crate::game::GameEffect;
use crate::models::asset::AssetType;

#[derive(Debug, Clone)]
pub struct Deck {
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    pub fn from_catalog(catalog: Vec<Card>) -> Self {
        let draw_pile = catalog.clone();
        let discard_pile = Vec::new();
        let _rng = rand::thread_rng();
        
        // Create deck from catalog
        Deck {
            draw_pile,
            discard_pile,
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        // Determine deck type based on first card's effect
        let deck_type = if !self.draw_pile.is_empty() {
            match &self.draw_pile[0].effect {
                GameEffect::OptionalBuyAsset { .. } => "Option to Buy",
                GameEffect::Expense(..) | GameEffect::ExpensePerAsset { .. } | GameEffect::PayIfNoAssetDistribute { .. } => "Operating Cost",
                _ => "Farmer's Fate",
            }
        } else if !self.discard_pile.is_empty() {
            match &self.discard_pile[0].effect {
                GameEffect::OptionalBuyAsset { .. } => "Option to Buy",
                GameEffect::Expense(..) | GameEffect::ExpensePerAsset { .. } | GameEffect::PayIfNoAssetDistribute { .. } => "Operating Cost",
                _ => "Farmer's Fate",
            }
        } else {
            "Empty"
        };

        if self.draw_pile.is_empty() {
            // If draw pile is empty, shuffle discard pile into draw pile
            if !self.discard_pile.is_empty() {
                println!("\nDraw pile empty, shuffling {} cards from discard pile", self.discard_pile.len());
                self.draw_pile.append(&mut self.discard_pile);
                self.shuffle();
            } else {
                println!("\nBoth draw pile and discard pile are empty for {} deck", deck_type);
                return None;
            }
        }
        
        // Remove from beginning and ensure we're not duplicating cards
        let card = self.draw_pile.remove(0);
        // Verify this card isn't in the discard pile
        if self.discard_pile.iter().any(|c| c.id == card.id) {
            println!("WARNING: Found duplicate card ID {} in discard pile!", card.id);
        }
        Some(card)
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    pub fn shuffle(&mut self) {
        // Determine deck type for printing
        let deck_type = if !self.draw_pile.is_empty() {
            match &self.draw_pile[0].effect {
                GameEffect::OptionalBuyAsset { .. } => "Option to Buy",
                GameEffect::Expense(..) | GameEffect::ExpensePerAsset { .. } | GameEffect::PayIfNoAssetDistribute { .. } => "Operating Cost",
                _ => "Farmer's Fate",
            }
        } else {
            "Empty"
        };

        if self.draw_pile.is_empty() {
             println!("Cannot shuffle an empty deck ({})", deck_type);
             return;
        }
        
        println!("Shuffling {} deck of {} cards", deck_type, self.draw_pile.len());

        let mut rng = rand::thread_rng();

        // For Option to Buy deck, shuffle and check for excessive clumping, reshuffle up to 5 times.
        if matches!(deck_type, "Option to Buy") {
            const MAX_SHUFFLE_ATTEMPTS: u32 = 5;
            let mut is_clumpy = true;
            let mut attempts = 0;

            while is_clumpy && attempts < MAX_SHUFFLE_ATTEMPTS {
                attempts += 1;
                self.draw_pile.shuffle(&mut rng);

                // Check distribution in top 20 cards only if deck is large enough
                if self.draw_pile.len() >= 20 {
                    let mut ridge_count = 0;
                    let mut land_count = 0;
                    let mut equipment_count = 0;
                    let mut other_count = 0;

                    for card in self.draw_pile.iter().take(20) {
                        match &card.effect {
                            GameEffect::OptionalBuyAsset { asset, .. } => match asset {
                                AssetType::Grain | AssetType::Hay | AssetType::Fruit => land_count += 1,
                                AssetType::Tractor | AssetType::Harvester => equipment_count += 1,
                                AssetType::Cows => other_count += 1, // Cows OTB are 'Other'
                            },
                            GameEffect::LeaseRidge { .. } => ridge_count += 1,
                            _ => other_count += 1, // Non-OTB/Lease cards are 'Other'
                        }
                    }

                    // Define "too clumpy": more than 9 ridge/land, or more than 7 equip/other in top 20
                    // Ideal counts in top 20 are roughly: Ridge 6, Land 6, Equip 4, Other 4
                    is_clumpy = ridge_count > 9 || land_count > 9 || equipment_count > 7 || other_count > 7;
                    
                    if attempts > 1 {
                         println!("  Shuffle attempt {}, Top 20 counts: R={}, L={}, E={}, O={}. Clumpy: {}", 
                                  attempts, ridge_count, land_count, equipment_count, other_count, is_clumpy);
                    }
                    
                    if !is_clumpy {
                        break; // Found a non-clumpy shuffle
                    }
                } else {
                    // If deck is too small to check top 20, just shuffle once
                    is_clumpy = false; 
                    break;
                }
            }
            
            if attempts == MAX_SHUFFLE_ATTEMPTS && is_clumpy {
                println!("  Reached max shuffle attempts ({}), accepting potentially clumpy distribution.", MAX_SHUFFLE_ATTEMPTS);
            }

            // Print top cards for verification
            println!("Top 6 cards after shuffle:");
            for (i, card) in self.draw_pile.iter().take(6).enumerate() {
                println!("  {}. {}", i + 1, card.title);
            }

        } else {
            // For other decks, just perform a single standard shuffle
            self.draw_pile.shuffle(&mut rng);
        }
    }
} 