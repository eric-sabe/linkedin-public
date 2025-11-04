use crate::game::GameEffect;

#[derive(Debug, Clone, PartialEq)]
pub enum CardSource {
    BaseGame,
    Expansion,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub description_brief: String,
    pub effect: GameEffect,
    pub default_quantity: u32,
    pub source: CardSource,
} 