use std::collections::BTreeMap;
use std::panic;
use std::rc::Rc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use yew::prelude::*;

use barohead_data::items::*;

mod widgets;

use crate::widgets::TextInput;

type ItemID = String;

// Statically compute a bunch of indexes and so on that we will use a bunch.
#[derive(Debug, PartialEq)]
struct AmbientData {
    items: BTreeMap<ItemID, Rc<Item>>,
    translations: ItemTranslations,
    items_by_description: BTreeMap<String, Rc<Item>>,
}

#[derive(Debug, PartialEq)]
struct ItemTranslations {
    texts: BTreeMap<String, String>,
}

impl ItemTranslations {
    pub fn get_name(&self, item: &Item) -> &str {
        let name_string = item.name_text_key();
        self.texts
            .get(name_string.as_str())
            .expect("Item translation")
    }
}

#[function_component[ItemSearch]]
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
    let matching_items: Vec<_> = ambient_data
        .items_by_description
        .iter()
        .filter_map(|(desc, item)| {
            let guess = (*search_input).as_str();
            let r#match = matcher.fuzzy_indices(desc, guess);
            r#match.map(|x| (desc, x, item))
        })
        .collect();

    html! {
        <>
            <TextInput value={(*search_input).clone()} {on_change}></TextInput>
            <pre>
                {format!("{:#?}", matching_items)}
            </pre>
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

// #[function_component(ItemLookup)]
// fn item_lookup() -> Html {
//     let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
//     html! {
//         <ItemList items={ambient_data.items.values().cloned().collect::<Vec<Rc<Item>>>()} />
//     }
// }

#[function_component(App)]
fn app() -> Html {
    // let items: UseStateHandle<Rc<Vec<Item>>> = use_state(|| Rc::new(vec![]));
    let ambient_data = use_memo(
        |_| {
            let items_bincode = std::include_bytes!("../recipes.bincode");
            let mut itemdb: ItemDB = bincode::deserialize(items_bincode).unwrap();

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

            AmbientData {
                items: rc_items,
                translations,
                items_by_description,
            }
        },
        (),
    );
    html! {
        <>
            <ContextProvider<Rc<AmbientData>> context={ambient_data}>
                <h1>{ "Barotruma Item Database" }</h1>
                <ItemSearch />
                // <ItemLookup />
            </ContextProvider<Rc<AmbientData>>>
        </>
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::Renderer::<App>::new().render();
}
