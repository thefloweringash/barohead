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
    pub nameidentifier: Option<String>,
    pub fabricate: Vec<Fabricate>,
    pub deconstruct: Vec<Deconstruct>,
    pub price: Option<Price>,
}

impl Item {
    pub fn name_text_key(&self) -> String {
        let id = self.nameidentifier.as_ref().unwrap_or(&self.id);
        format!("entityname.{}", id)
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy, Ord, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StoreIdentifier {
    MerchantOutpost,
    MerchantCity,
    MerchantResearch,
    MerchantMilitary,
    MerchantMine,
    MerchantMedical,
    MerchantEngineering,
    MerchantArmory,
    MerchantClown,
    MerchantHusk,
    MerchantTutorial,
}

impl StoreIdentifier {
    pub fn internal_name(self) -> &'static str {
        match self {
            StoreIdentifier::MerchantOutpost => "merchantoutpost",
            StoreIdentifier::MerchantCity => "merchantcity",
            StoreIdentifier::MerchantResearch => "merchantresearch",
            StoreIdentifier::MerchantMilitary => "merchantmilitary",
            StoreIdentifier::MerchantMine => "merchantmine",
            StoreIdentifier::MerchantMedical => "merchantmedical",
            StoreIdentifier::MerchantEngineering => "merchantengineering",
            StoreIdentifier::MerchantArmory => "merchantarmory",
            StoreIdentifier::MerchantClown => "merchantclown",
            StoreIdentifier::MerchantHusk => "merchanthusk",
            StoreIdentifier::MerchantTutorial => "merchanttutorial",
        }
    }

    pub fn name_text_key(&self) -> String {
        format!("storename.{}", self.internal_name())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PriceModifier {
    pub multiplier: Option<f32>,
    pub sold: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Price {
    pub baseprice: i32,
    pub sold: bool,
    pub modifiers: BTreeMap<StoreIdentifier, PriceModifier>,
}
