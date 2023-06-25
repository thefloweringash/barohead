use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemDB {
    pub texts: BTreeMap<Language, BTreeMap<String, String>>,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Language {
    English,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub fabricate: Vec<Fabricate>,
    pub deconstruct: Vec<Deconstruct>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Fabricate {
    pub suitable_fabricators: Vec<Fabricator>,
    pub time: f32,
    pub required_items: Vec<RequiredItem>,
    pub required_skills: BTreeMap<Skill, i32>,
    pub requires_recipe: bool,
    pub out_condition: f32,
    pub amount: i32,
    pub recycle: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Deconstruct {
    pub time: f32,
    pub required_items: Vec<RequiredItem>,
    pub required_skills: BTreeMap<Skill, i32>,
    pub items: Vec<ProducedItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequiredItem {
    pub item: ItemRef,
    pub amount: i32,
    pub condition: Option<ConditionRange>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProducedItem {
    pub id: String,
    pub amount: i32,
    pub mincondition: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConditionRange {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Fabricator {
    Fabricator,
    MedicalFabricator,
    VendingMachine,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    Engineering,
    Electrical,
    Medical,
    Mechanical,
    Weapons,
    Helm,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemRef {
    Tag(String),
    Id(String),
}
