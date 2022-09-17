use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Stat {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Stats {
    pub base_stat: i32,
    pub effort: i32,
    pub stat: Stat,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Form {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Sprites {
    pub front_default: String,
    pub back_default: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub weight: u32,
    pub sprites: Sprites,
    pub order: u32,
    pub stats: Vec<Stats>,
    pub forms: Vec<Form>,
}
