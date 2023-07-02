use std::collections::BTreeMap;
use std::rc::Rc;

use barohead_data::items as data;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use string_interner::StringInterner;

// Statically compute a bunch of indexes and so on that we will use a bunch.

type ItemID = string_interner::DefaultSymbol;

#[derive(Debug, PartialEq, Clone)]
pub struct SearchResult {
    pub item_ref: ItemRef,
    pub score: i64,
    pub indices: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct ItemRef {
    item_id: ItemID,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FabricateRef {
    pub item_ref: ItemRef,
    pub idx: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeconstructRef {
    pub item_ref: ItemRef,
    pub idx: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessRef {
    Fabricate(FabricateRef),
    Deconstruct(DeconstructRef),
}

#[derive(Debug, PartialEq)]
pub struct AmbientData {
    item_ids: StringInterner,

    items: BTreeMap<ItemID, Rc<data::Item>>,
    items_used_by: ProcessIndex,
    items_produced_by: ProcessIndex,

    pub translations: ItemTranslations,
}

type ProcessIndex = BTreeMap<ItemID, Rc<Vec<ProcessRef>>>;

pub struct IndexBuilder<'a> {
    item_ids: &'a StringInterner,
    map: BTreeMap<ItemID, Vec<ProcessRef>>,
}

impl<'a> IndexBuilder<'a> {
    pub fn new(item_ids: &'a StringInterner) -> Self {
        Self {
            item_ids,
            map: Default::default(),
        }
    }

    pub fn add_reference(&mut self, id: &str, process_ref: &ProcessRef) {
        let item_id = self.item_ids.get(id).unwrap();
        if let Some(refs) = self.map.get_mut(&item_id) {
            if !refs.contains(process_ref) {
                refs.push(process_ref.clone())
            }
        } else {
            self.map.insert(item_id, vec![process_ref.clone()]);
        }
    }

    pub fn extract(self) -> ProcessIndex {
        self.map
            .into_iter()
            .map(|(id, refs)| (id, Rc::new(refs)))
            .collect()
    }
}

fn build_indexes(
    item_ids: &StringInterner,
    items: &BTreeMap<ItemID, Rc<data::Item>>,
) -> (ProcessIndex, ProcessIndex) {
    let mut used_by_builder = IndexBuilder::new(item_ids);
    let mut produced_by_builder = IndexBuilder::new(item_ids);

    for (item_id, item) in items.iter() {
        let item_ref = ItemRef { item_id: *item_id };
        for (idx, fabricate) in item.fabricate.iter().enumerate() {
            let process_ref = ProcessRef::Fabricate(FabricateRef { item_ref, idx });

            for required_item in &fabricate.required_items {
                if let data::ItemRef::Id(id) = &required_item.item {
                    used_by_builder.add_reference(id, &process_ref);
                }
            }
        }

        for (idx, deconstruct) in item.deconstruct.iter().enumerate() {
            let process_ref = ProcessRef::Deconstruct(DeconstructRef { item_ref, idx });

            for required_item in &deconstruct.required_items {
                if let data::ItemRef::Id(id) = &required_item.item {
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
    pub fn from(mut itemdb: data::ItemDB) -> Self {
        let mut item_ids = StringInterner::default();

        let rc_items: BTreeMap<ItemID, Rc<data::Item>> = itemdb
            .items
            .into_iter()
            .map(|item| (item_ids.get_or_intern(&item.id), Rc::new(item)))
            .collect();

        let english_texts = itemdb
            .texts
            .remove(&data::Language::English)
            .unwrap()
            .into_iter()
            .map(|(key, translation)| (key, Rc::from(translation)))
            .collect();

        let translations = ItemTranslations {
            texts: english_texts,
        };

        let (items_used_by, items_produced_by) = build_indexes(&item_ids, &rc_items);

        Self {
            item_ids,
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
            .iter()
            .filter_map(|(item_id, item)| {
                let description = &self.translations.get_name_rc(item);

                let description = description.as_ref().map(|x| x.as_str()).unwrap_or(&item.id);
                matcher
                    .fuzzy_indices(description, query)
                    .map(|(score, indices)| SearchResult {
                        item_ref: ItemRef { item_id: *item_id },
                        score,
                        indices,
                    })
            })
            .collect::<Vec<_>>();

        matching_items.sort_by(|a, b| b.score.cmp(&a.score));

        matching_items
    }

    pub fn get_item(&self, item_ref: ItemRef) -> &data::Item {
        let item = self.items.get(&item_ref.item_id);
        item.unwrap()
    }

    pub fn new_item_ref(&self, id_str: &str) -> Option<ItemRef> {
        self.item_ids.get(id_str).map(|item_id| ItemRef { item_id })
    }

    pub fn get_fabricate<'a>(&'a self, fabricate_ref: &FabricateRef) -> &'a data::Fabricate {
        let item = self.items.get(&fabricate_ref.item_ref.item_id).unwrap();
        item.fabricate.get(fabricate_ref.idx).unwrap()
    }

    pub fn get_deconstruct<'a>(
        &'a self,
        deconstruct_ref: &DeconstructRef,
    ) -> &'a data::Deconstruct {
        let item = self.items.get(&deconstruct_ref.item_ref.item_id).unwrap();
        item.deconstruct.get(deconstruct_ref.idx).unwrap()
    }

    pub fn get_used_by(&self, item_ref: ItemRef) -> Option<Rc<Vec<ProcessRef>>> {
        self.items_used_by.get(&item_ref.item_id).cloned()
    }

    pub fn get_produced_by(&self, item_ref: ItemRef) -> Option<Rc<Vec<ProcessRef>>> {
        self.items_produced_by.get(&item_ref.item_id).cloned()
    }
}

#[derive(Debug, PartialEq)]
pub struct ItemTranslations {
    texts: BTreeMap<String, Rc<String>>,
}

impl ItemTranslations {
    pub fn get_name<'a>(&'a self, item: &'a data::Item) -> &'a str {
        let name_string = item.name_text_key();

        self.texts
            .get(name_string.as_str())
            .map(|x| x.as_str())
            .unwrap_or(item.id.as_str())
    }

    pub fn get_name_rc(&self, item: &data::Item) -> Option<Rc<String>> {
        let name_string = item.name_text_key();

        self.texts.get(name_string.as_str()).cloned()
    }
}
