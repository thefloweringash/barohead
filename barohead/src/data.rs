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

#[derive(Debug, PartialEq)]
pub struct AmbientData {
    items: BTreeMap<String, Rc<Item>>,

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

        Self {
            items: rc_items,
            translations,
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
