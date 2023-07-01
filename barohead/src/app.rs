use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::*;

use crate::data::{AmbientData, SearchResult};
use crate::widgets::TextInput;

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

    let results = {
        ambient_data
            .search(&*search_input)
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
