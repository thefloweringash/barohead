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
                html! { <span key={idx} class="match">{ch}</span> }
            }
            _ => {
                html! { <span key={idx}>{ch}</span> }
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div>
            <h2>{visible_match}</h2>
            <ItemView key={search_result.item.id.as_str()} item={search_result.item.clone()} />
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
            <div class="search-results">
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

    let fabricate = item
        .fabricate
        .iter()
        .map(|fabricate| {
            let required_items = fabricate
                .required_items
                .iter()
                .map(|required_item| {
                    match &required_item.item {
                        ItemRef::Id(id) => {
                            let item = ambient_data.get_item(id).expect("Fabricate Required item");
                            html! {
                                <ItemThumbnail
                                    {item}
                                    amount={required_item.amount}
                                    condition_range={required_item.condition.clone()}
                                />
                            }
                        }
                        ItemRef::Tag(tag) => {
                            html! {
                                <div>
                                    {"(Tag) "}
                                    if required_item.amount != 1 {
                                      <span class="amount">{required_item.amount}</span>
                                    }
                                    <span class="name">{tag}</span>
                                    if required_item.condition.is_some() {
                                        <span class="condition">{format!("{:#?}", required_item.condition)}</span>
                                    }
                                </div>
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();
            html! {
                <div class="fabricate">
                    <div class="required-items">{required_items}</div>
                    <div class="production-arrow">{"->"}</div>
                    <div class="produced-items">
                        <ItemThumbnail {item} amount={fabricate.amount} condition={fabricate.out_condition} />
                    </div>
                </div>
            }
        })
        .collect::<Vec<_>>();

    let deconstruct = item
        .deconstruct
        .iter()
        .map(|deconstruct| {
            let required_items = deconstruct
                .required_items
                .iter()
                .map(|required_item| match &required_item.item {
                    ItemRef::Id(id) => {
                        let item = ambient_data
                            .get_item(id.as_str())
                            .expect("Deconstruct Required item");
                        html! {
                            <ItemThumbnail
                                {item}
                                amount={required_item.amount}
                                condition_range={required_item.condition.clone()}
                            />
                        }
                    }
                    _ => {
                        panic!("Does this really happen?");
                    }
                })
                .collect::<Vec<_>>();
            let produced_items = deconstruct
                .items
                .iter()
                .map(|produced_item| {
                    let item = ambient_data
                        .get_item(produced_item.id.as_str())
                        .expect("Deconstruct Produced item");
                    // TODO: The produced items are conditional based on input condition.
                    html! {
                        <ItemThumbnail
                            {item}
                            amount={produced_item.amount}
                        />
                    }
                })
                .collect::<Vec<_>>();
            html! {
                <div class="deconstruct">
                    <div class="required-items">
                        <ItemThumbnail {item} />
                        {required_items}
                    </div>
                    <div class="production-arrow">{"->"}</div>
                    <div class="produced-items">{produced_items}</div>
                </div>
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div>
            <h3>{"Details"}</h3>
            <dl>
                <dt>{"Id"}</dt>
                <dd>{id}</dd>
            </dl>
            <h3>{format!("Fabricated By ({})", fabricate.len())}</h3>
            {fabricate}
            <h3>{format!("Deconstructs Into ({})", deconstruct.len())}</h3>
            {deconstruct}
            <h3>{"Used by"}</h3>
            <div>{"TODO"}</div>
            <h3>{"Produced by"}</h3>
            <div>{"TODO"}</div>
            <h3>{"Debug"}</h3>
            <details>
               <summary>{"Raw data"}</summary>

                <pre>
                    {format!("{:#?}", item)}
                </pre>
            </details>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ItemThumbnailProps {
    item: Rc<Item>,
    #[prop_or_default]
    amount: Option<i32>,
    #[prop_or_default]
    condition_range: Option<ConditionRange>,
    #[prop_or_default]
    condition: Option<f32>,
}
#[function_component(ItemThumbnail)]
fn item_thumbnail(
    ItemThumbnailProps {
        item,
        amount,
        condition,
        condition_range,
    }: &ItemThumbnailProps,
) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();

    // let id = item.id.as_str();

    // let name = ambient_data.translations.get_name(item);

    html! {
        <div class="item-thumbnail">
            if amount.is_some() && amount.unwrap() != 1 {
              <span class="amount">{amount.unwrap()} {"x"}</span>
            }
            { " " }
            <span class="name">{ambient_data.translations.get_name(item)}</span>
            if condition_range.is_some() {
                <span class="condition-range">{format!("{:#?}", condition_range)}</span>
            }
            if condition.is_some() && condition.unwrap() != 1.0 {
                <span class="conditione">{format!("{:#?}", condition)}</span>
            }
        </div>
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
    // It might be easier to make this a lazy_static, but then we don't have the
    // option of having different databases for different versions.
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
