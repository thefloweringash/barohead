use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

use barohead_data::items::*;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

// Statically compute a bunch of indexes and so on that we will use a bunch.

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub description: String,
    pub score: i64,
    pub indices: Vec<usize>,
    pub item: Rc<Item>,
}

#[derive(Debug, PartialEq)]
pub struct AmbientData {
    items: BTreeMap<String, Rc<Item>>,
    items_by_description: BTreeMap<String, Rc<Item>>,

    pub translations: ItemTranslations,
}

impl AmbientData {
    pub fn from(mut itemdb: ItemDB) -> Self {
        let rc_items: BTreeMap<String, Rc<Item>> = itemdb
            .items
            .into_iter()
            .map(|item| (item.id.to_owned(), Rc::new(item)))
            .collect();

        let english_texts = itemdb.texts.remove(&Language::English).unwrap();

        let translations = ItemTranslations {
            texts: english_texts,
        };

        let items_by_description = rc_items
            .values()
            .map(|item| (format!("{}", translations.get_name(item)), item.clone()))
            .collect();

        Self {
            items: rc_items,
            translations,
            items_by_description,
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let matcher = SkimMatcherV2::default();
        let mut matching_items: Vec<_> = self
            .items_by_description
            .iter()
            .filter_map(|(description, item)| {
                let description = description.as_str();
                matcher
                    .fuzzy_indices(description, query)
                    .map(|(score, indices)| SearchResult {
                        description: description.to_owned(),
                        score,
                        indices,
                        item: item.clone(),
                    })
            })
            .collect::<Vec<_>>();

        matching_items.sort_by(|a, b| b.score.cmp(&a.score));

        matching_items
    }

    pub fn get_item(&self, item_id: &str) -> Option<Rc<Item>> {
        self.items.get(item_id).map(|item| item.clone())
    }
}

#[derive(Debug, PartialEq)]
pub struct ItemTranslations {
    texts: BTreeMap<String, String>,
}

impl ItemTranslations {
    pub fn get_name<'a>(&'a self, item: &'a Item) -> &'a str {
        let name_string = item.name_text_key();

        self.texts
            .get(name_string.as_str())
            .map(|x| x.as_str())
            .unwrap_or(item.id.as_str())
    }
}

enum ActiveItemRef {
    Tag(String),
    Item(Weak<ActiveItem>),
}

impl ActiveItemRef {
    pub fn is_item(&self) -> bool {
        match self {
            Self::Item(_) => true,
            _ => false,
        }
    }

    pub fn get_item(&self) -> Rc<ActiveItem> {
        match self {
            Self::Item(item) => item.upgrade().unwrap(),
            _ => panic!("Not an item"),
        }
    }
}

pub struct ActiveRequiredItem {
    item: ActiveItemRef,
    pub amount: i32,
    pub condition: Option<ConditionRange>,
}

struct ActiveFabricate {
    pub suitable_fabricators: Vec<Fabricator>,
    pub time: f32,
    pub required_items: Vec<ActiveRequiredItem>,
    pub required_skills: BTreeMap<Skill, i32>,
    pub requires_recipe: bool,
    pub out_condition: f32,
    pub amount: i32,
    pub recycle: bool,
    produced_item: Weak<ActiveItem>,
}

impl ActiveFabricate {
    pub fn produced_item(&self) -> Rc<ActiveItem> {
        self.produced_item.upgrade().unwrap()
    }
}

struct ActiveDeconstruct {
    pub deconstruct: Deconstruct,
    required_items: Vec<Weak<ActiveItem>>,
}

enum ProcessRef {
    Fabricate(Rc<ActiveFabricate>),
    Deconstruct(Rc<ActiveDeconstruct>),
}

struct ActiveItem {
    name: String,
    fabricate: Vec<Rc<ActiveFabricate>>,
    deconstruct: Vec<Rc<ActiveDeconstruct>>,
    used_by: Vec<ProcessRef>,
    produced_by: Vec<ProcessRef>,
}

struct ActiveDB {
    active_items: BTreeMap<String, Rc<ActiveItem>>,
}

impl ActiveDB {
    pub fn from(mut db: ItemDB, translations: &ItemTranslations) -> Self {
        let mut active_items = db
            .items
            .iter()
            .map(|item| {
                let name = translations.get_name(&item).to_owned();
                let active_item = ActiveItem {
                    name: name.clone(),
                    fabricate: vec![],
                    deconstruct: vec![],
                    used_by: vec![],
                    produced_by: vec![],
                };
                (name, Rc::new(active_item))
            })
            .collect::<BTreeMap<_, _>>();

        // We have an item we can point two in assembling our other models
        for item in db.items.into_iter() {
            let active_fabricates = item
                .fabricate
                .iter()
                .map(|fabricate| {
                    Rc::new(ActiveFabricate {
                        suitable_fabricators: fabricate.suitable_fabricators.clone(),
                        time: fabricate.time,
                        required_items: fabricate
                            .required_items
                            .iter()
                            .map(|x| {
                                let item_ref = match &x.item {
                                    ItemRef::Tag(str) => ActiveItemRef::Tag(str.clone()),
                                    ItemRef::Id(id) => ActiveItemRef::Item(Rc::downgrade(
                                        active_items.get(id).unwrap(),
                                    )),
                                };
                                ActiveRequiredItem {
                                    item: item_ref,
                                    amount: x.amount,
                                    condition: x.condition.clone(),
                                }
                            })
                            .collect(),
                        required_skills: fabricate.required_skills.clone(),
                        requires_recipe: fabricate.requires_recipe,
                        out_condition: fabricate.out_condition,
                        amount: fabricate.amount,
                        recycle: fabricate.recycle,
                        produced_item: Rc::downgrade(active_items.get(&item.id).unwrap()),
                    })
                })
                .collect::<Vec<_>>();
            let mut active_item = active_items.get_mut(&item.id).unwrap();
            active_item.get_mut().fabricate.extend(active_fabricates);
        }

        ActiveDB { active_items }
    }
}
