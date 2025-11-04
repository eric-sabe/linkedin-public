use crate::models::{GameState, AssetType, AssetRecord, PlayerType};

impl GameState {
    pub fn run_bankruptcy_auction(&mut self, player_id: usize) {
        let player = self.players.get(&player_id).unwrap();
        let mut assets: Vec<(AssetType, AssetRecord)> = player.assets.iter()
            .map(|(asset_type, record)| (*asset_type, record.clone()))
            .collect();
        
        // Sort assets by value (highest first)
        assets.sort_by(|a, b| b.1.total_cost.cmp(&a.1.total_cost));

        for (asset_type, record) in assets {
            let total_value = record.total_cost;
            println!("\nAuctioning {} (Quantity: {}, Value: ${})", 
                format!("{:?}", asset_type), record.quantity, total_value);

            let mut highest_bid = 0;
            let mut highest_bidder = None;

            // Run auction among other players
            for (other_id, other_player) in self.players.iter() {
                if *other_id != player_id && other_player.cash > highest_bid {
                    // AI players bid based on their cash and asset value
                    if let PlayerType::AI(_) = other_player.player_type {
                        let bid = (other_player.cash as f32 * 0.8) as i32;
                        if bid > highest_bid {
                            highest_bid = bid;
                            highest_bidder = Some(*other_id);
                        }
                    } else {
                        println!("{} has ${}. Enter bid (0 to pass): ", other_player.name, other_player.cash);
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let bid: i32 = input.trim().parse().unwrap_or(0);
                        if bid > highest_bid && bid <= other_player.cash {
                            highest_bid = bid;
                            highest_bidder = Some(*other_id);
                        }
                    }
                }
            }

            if let Some(bidder_id) = highest_bidder {
                // Transfer asset to highest bidder
                let bidder = self.players.get_mut(&bidder_id).unwrap();
                bidder.cash -= highest_bid;
                bidder.add_asset(asset_type, record.quantity, highest_bid);
                
                println!("{} won the auction for {} with a bid of ${}", 
                    bidder.name, format!("{:?}", asset_type), highest_bid);
            } else {
                println!("No bids received for {}", format!("{:?}", asset_type));
            }
        }
    }

    pub fn attempt_bank_loan(&mut self, player_id: usize) -> bool {
        let player = self.players.get(&player_id).unwrap();
        let total_asset_value: i32 = player.assets.values()
            .map(|record| record.total_cost)
            .sum();
        
        let max_loan = total_asset_value / 2;
        
        // AI players automatically accept the loan
        // Human players are prompted
        let loan_amount = if let PlayerType::AI(_) = player.player_type {
            max_loan
        } else {
            println!("Bank offers a loan of ${}. Accept? (y/n): ", max_loan);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                max_loan
            } else {
                0
            }
        };

        if loan_amount > 0 {
            let player = self.players.get_mut(&player_id).unwrap();
            player.cash += loan_amount;
            player.debt += loan_amount;
            println!("Loan of ${} accepted. New debt: ${}", loan_amount, player.debt);
            true
        } else {
            false
        }
    }

    pub fn check_bankruptcy_and_trigger_auction(&mut self, player_id: usize) {
        let player = self.players.get(&player_id).unwrap();
        if player.cash < 0 {
            println!("\n{} is bankrupt!", player.name);
            
            // Try to get a bank loan first
            if self.attempt_bank_loan(player_id) {
                return;
            }
            
            // If no loan or loan refused, run the auction
            println!("Starting bankruptcy auction...");
            self.run_bankruptcy_auction(player_id);
        }
    }
} 