use std::collections::BTreeMap;
use std::rc::Rc;

use barohead_data::items::*;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

// Statically compute a bunch of indexes and so on that we will use a bunch.

#[derive(Debug, PartialEq, Clone)]
pub struct SearchResult {
    pub description: String,
    pub score: i64,
    pub indices: Vec<usize>,
    pub item: Rc<Item>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FabricateRef {
    pub item_id: String,
    pub idx: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeconstructRef {
    pub item_id: String,
    pub idx: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessRef {
    Fabricate(FabricateRef),
    Deconstruct(DeconstructRef),
}

#[derive(Debug, PartialEq)]
pub struct AmbientData {
    items: BTreeMap<String, Rc<Item>>,
    items_used_by: ProcessIndex,
    items_produced_by: ProcessIndex,

    pub translations: ItemTranslations,
}

type ProcessIndex = BTreeMap<String, Rc<Vec<ProcessRef>>>;

pub struct IndexBuilder {
    map: BTreeMap<String, Vec<ProcessRef>>,
}

impl IndexBuilder {
    pub fn new() -> Self {
        let map = BTreeMap::<String, Vec<ProcessRef>>::new();
        Self { map }
    }

    pub fn add_reference(&mut self, id: &str, process_ref: &ProcessRef) {
        if let Some(refs) = self.map.get_mut(id) {
            if !refs.contains(process_ref) {
                refs.push(process_ref.clone())
            }
        } else {
            self.map.insert(id.to_owned(), vec![process_ref.clone()]);
        }
    }

    pub fn extract(self) -> ProcessIndex {
        self.map
            .into_iter()
            .map(|(id, refs)| (id, Rc::new(refs)))
            .collect()
    }
}

fn build_indexes(items: &BTreeMap<String, Rc<Item>>) -> (ProcessIndex, ProcessIndex) {
    let mut used_by_builder = IndexBuilder::new();
    let mut produced_by_builder = IndexBuilder::new();

    for item in items.values() {
        for (idx, fabricate) in item.fabricate.iter().enumerate() {
            let process_ref = ProcessRef::Fabricate(FabricateRef {
                item_id: item.id.clone(),
                idx,
            });
            for required_item in &fabricate.required_items {
                if let ItemRef::Id(id) = &required_item.item {
                    used_by_builder.add_reference(id, &process_ref);
                }
            }
        }

        for (idx, deconstruct) in item.deconstruct.iter().enumerate() {
            let process_ref = ProcessRef::Deconstruct(DeconstructRef {
                item_id: item.id.clone(),
                idx,
            });
            for required_item in &deconstruct.required_items {
                if let ItemRef::Id(id) = &required_item.item {
                    used_by_builder.add_reference(id, &process_ref);
                }
            }

            for produced_item in &deconstruct.items {
                produced_by_builder.add_reference(&produced_item.id, &process_ref);
            }
        }
    }

    // Throw away the boxes,
    (used_by_builder.extract(), produced_by_builder.extract())
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

        let (items_used_by, items_produced_by) = build_indexes(&rc_items);

        Self {
            items: rc_items,
            translations,
            items_used_by,
            items_produced_by,
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let matcher = SkimMatcherV2::default();
        let mut matching_items: Vec<_> = self
            .items
            .values()
            .filter_map(|item| {
                let description = self.translations.get_name(item);
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
        self.items.get(item_id).cloned()
    }

    pub fn get_used_by(&self, item_id: &str) -> Option<Rc<Vec<ProcessRef>>> {
        self.items_used_by.get(item_id).cloned()
    }

    pub fn get_produced_by(&self, item_id: &str) -> Option<Rc<Vec<ProcessRef>>> {
        self.items_produced_by.get(item_id).cloned()
    }

    pub fn get_fabricate<'a>(&'a self, fabricate_ref: &FabricateRef) -> &'a Fabricate {
        let item = self.items.get(fabricate_ref.item_id.as_str()).unwrap();
        item.fabricate.get(fabricate_ref.idx).unwrap()
    }

    pub fn get_deconstruct<'a>(&'a self, deconstruct_ref: &DeconstructRef) -> &'a Deconstruct {
        let item = self.items.get(deconstruct_ref.item_id.as_str()).unwrap();
        item.deconstruct.get(deconstruct_ref.idx).unwrap()
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
