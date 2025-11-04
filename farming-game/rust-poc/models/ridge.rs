#[derive(Debug, Clone)]
pub struct Ridge {
    pub name: String,
    pub cost: i32,
    pub cow_count: i32,
    pub leased_by: Option<usize>,
    pub initial_cow_count: i32,
}

impl Ridge {
    pub fn new(name: String, cost: i32, initial_cow_count: i32) -> Self {
        Self {
            name,
            cost,
            cow_count: 0,
            leased_by: None,
            initial_cow_count,
        }
    }

    pub fn can_add_cows(&self, amount: i32) -> bool {
        self.cow_count + amount <= self.initial_cow_count
    }

    pub fn add_cows(&mut self, amount: i32) -> Result<(), String> {
        if self.can_add_cows(amount) {
            self.cow_count += amount;
            Ok(())
        } else {
            Err(format!("Cannot add {} cows. Ridge capacity is {}.", amount, self.initial_cow_count))
        }
    }

    pub fn remove_cows(&mut self, amount: i32) -> Result<(), String> {
        if amount <= self.cow_count {
            self.cow_count -= amount;
            Ok(())
        } else {
            Err(format!("Cannot remove {} cows. Only {} cows present.", amount, self.cow_count))
        }
    }

    pub fn lease(&mut self, player_id: usize, initial_cows: i32) -> Result<(), String> {
        if self.leased_by.is_some() {
            return Err("Ridge is already leased.".to_string());
        }
        if initial_cows != self.initial_cow_count {
            return Err(format!("Invalid initial cow count. Ridge requires {} cows.", self.initial_cow_count));
        }
        self.leased_by = Some(player_id);
        self.cow_count = initial_cows;
        Ok(())
    }

    pub fn is_leased(&self) -> bool {
        self.leased_by.is_some()
    }

    pub fn get_leasee(&self) -> Option<usize> {
        self.leased_by
    }
} 