use rand::Rng;
use crate::models::AssetType;

impl GameState {
    fn handle_mt_st_helens_disaster(&mut self, player_id: &str, logs: &mut Vec<String>) -> Result<(), String> {
        // First, give the card holder $500 per Hay acre
        let card_holder = match self.players.get_mut(player_id) {
            Some(player) => player,
            None => return Err(format!("Player with ID {} not found.", player_id)),
        };

        if let Some(hay_record) = card_holder.assets.get(&AssetType::Hay) {
            let bonus = hay_record.quantity * 500;
            card_holder.cash += bonus;
            logs.push(format!("{} collects ${} bonus for {} Hay acres (Mt. St. Helens price jump).", 
                card_holder.name, bonus, hay_record.quantity));
        }

        // Then, handle other players' rolls and potential expenses
        for (other_id, other_player) in self.players.iter_mut() {
            if *other_id != player_id {
                // Roll for each other player (Odd=escaped, Even=hit)
                let roll = rand::thread_rng().gen_range(1..=6);
                let escaped = roll % 2 == 1;
                
                if escaped {
                    logs.push(format!("{} rolled {} (Odd) and escaped the ash!", other_player.name, roll));
                } else {
                    logs.push(format!("{} rolled {} (Even) and was hit by the ash!", other_player.name, roll));
                    
                    // Calculate total acres across all crops
                    let total_acres: i32 = other_player.assets.values()
                        .map(|record| record.quantity)
                        .sum();
                    
                    if total_acres > 0 {
                        let cleanup_cost = total_acres * 100;
                        logs.push(format!("{} must pay ${} to clean up ash (${} per acre).", 
                            other_player.name, cleanup_cost, 100));
                        self.handle_forced_loan(*other_id, cleanup_cost, logs)?;
                    } else {
                        logs.push(format!("{} has no acres to clean up.", other_player.name));
                    }
                }
            }
        }
        Ok(())
    }
} 