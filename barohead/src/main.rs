use std::collections::BTreeMap;
use std::panic;
use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::*;

#[derive(Debug, PartialEq)]
struct AmbientData {
    items: Rc<BTreeMap<String, Rc<Item>>>,
    texts: Rc<BTreeMap<String, String>>,
}

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Vec<Rc<Item>>,
}

#[function_component(ItemList)]
fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();

    items
        .iter()
        .map(|item| {
            let id = item.id.as_str();

            let name = ambient_data
                .texts
                .get(item.name_text_key().as_str())
                .map(|t| t.as_str())
                .unwrap_or(id);

            html! {
                <p key={id}>
                    {name}
                    <pre>
                        {format!("{:#?}", item)}
                    </pre>
                </p>
            }
        })
        .collect()
}

#[function_component(ItemLookup)]
fn item_lookup() -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    html! {
        <ItemList items={ambient_data.items.values().cloned().collect::<Vec<Rc<Item>>>()} />
    }
}

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

            AmbientData {
                items: Rc::new(rc_items),
                texts: Rc::new(english_texts),
            }
        },
        (),
    );
    html! {
        <>
            <ContextProvider<Rc<AmbientData>> context={ambient_data}>
                <h1>{ "Hello World" }</h1>
                <ItemLookup />
            </ContextProvider<Rc<AmbientData>>>
        </>
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::Renderer::<App>::new().render();
}
