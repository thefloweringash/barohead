use std::collections::BTreeMap;
use std::rc::Rc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use yew::prelude::*;

use barohead_data::items::*;

use crate::widgets::TextInput;

type ItemID = String;

// Statically compute a bunch of indexes and so on that we will use a bunch.
#[derive(Debug, PartialEq)]
struct AmbientData {
    items: BTreeMap<ItemID, Rc<Item>>,
    translations: ItemTranslations,
    items_by_description: BTreeMap<String, Rc<Item>>,
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
}

#[derive(Debug, PartialEq)]
struct ItemTranslations {
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

#[derive(Debug, PartialEq)]
struct SearchResult {
    description: String,
    score: i64,
    indices: Vec<usize>,
    item: Rc<Item>,
}

#[derive(Properties, PartialEq)]
struct ShowSearchResultProps {
    search_result: SearchResult,
}

#[function_component(ShowSearchResult)]
fn show_search_result(ShowSearchResultProps { search_result }: &ShowSearchResultProps) -> Html {
    let mut peekable = search_result.indices.iter().peekable();

    // TODO: this is _really_ slow, you can feel the difference. It should use chunks.
    let visible_match = search_result
        .description
        .char_indices()
        .map(|(idx, ch)| match peekable.peek() {
            Some(next_idx) if **next_idx == idx => {
                peekable.next();
                html! { <span key={idx} style="color: blue">{ch}</span> }
            }
            _ => {
                html! { <span key={idx}>{ch}</span> }
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div>
            <h2>{visible_match}</h2>
            <ItemView item={search_result.item.clone()} />
        </div>
    }
}

#[function_component(ItemSearch)]
fn item_search() -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let search_input = use_state(|| "".to_owned());

    let on_change = {
        let search_input = search_input.clone();
        Callback::from(move |e: String| {
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(e.as_str()));
            search_input.set(e)
        })
    };

    let matcher = SkimMatcherV2::default();
    let mut matching_items: Vec<_> = ambient_data
        .items_by_description
        .iter()
        .filter_map(|(description, item)| {
            let description = description.as_str();
            let guess = (*search_input).as_str();
            matcher
                .fuzzy_indices(description, guess)
                .map(|(score, indices)| SearchResult {
                    description: description.to_owned(),
                    score,
                    indices,
                    item: item.clone(),
                })
        })
        .collect::<Vec<_>>();

    matching_items.sort_by(|a, b| b.score.cmp(&a.score));

    let results = {
        matching_items
            .into_iter()
            .map(|search_result| {
                let key = search_result.item.id.to_owned();
                html! {
                    <ShowSearchResult {key} {search_result} />
                }
            })
            .collect::<Vec<_>>()
    };
    html! {
        <>
            <TextInput value={(*search_input).clone()} {on_change}></TextInput>
            <div>
                {results}
            </div>
        </>

    }
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: Rc<Item>,
}

#[function_component(ItemView)]
fn item_view(ItemProps { item }: &ItemProps) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();

    let id = item.id.as_str();

    let name = ambient_data.translations.get_name(item);

    html! {
        <p key={id}>
            {name}
            <pre>
                {format!("{:#?}", item)}
            </pre>
        </p>
    }
}

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Vec<Rc<Item>>,
}
#[function_component(ItemList)]
fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
    items
        .iter()
        .map(|item| {
            html! {
                <ItemView key={item.id.as_str()} item={item.clone()} />
            }
        })
        .collect()
}

#[function_component(App)]
pub fn app() -> Html {
    let ambient_data = use_memo(
        |_| {
            let items_bincode = std::include_bytes!("../recipes.bincode");
            let itemdb: ItemDB = bincode::deserialize(items_bincode).unwrap();
            AmbientData::from(itemdb)
        },
        (),
    );
    html! {
        <>
            <ContextProvider<Rc<AmbientData>> context={ambient_data}>
                <h1>{ "Barotruma Item Database" }</h1>
                <ItemSearch />
            </ContextProvider<Rc<AmbientData>>>
        </>
    }
}
