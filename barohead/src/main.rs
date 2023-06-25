use std::collections::BTreeMap;
use std::panic;
use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::*;

#[derive(Debug, PartialEq)]
struct AmbientData {
    items: Rc<Vec<Item>>,
    texts: Rc<BTreeMap<String, String>>,
}

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Rc<Vec<Item>>,
}

#[function_component(ItemList)]
fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();

    items
        .iter()
        .map(|item| {
            let id = item.id.as_str();

            let name_translation_key = format!("entityname.{}", id);

            let name = ambient_data
                .texts
                .get(name_translation_key.as_str())
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
        <ItemList items={ambient_data.items.clone()} />
    }
}

#[function_component(App)]
fn app() -> Html {
    // let items: UseStateHandle<Rc<Vec<Item>>> = use_state(|| Rc::new(vec![]));
    let ambient_data = use_memo(
        |_| {
            let items_bincode = std::include_bytes!("../recipes.bincode");
            let mut itemdb: ItemDB = bincode::deserialize(items_bincode).unwrap();

            let english_texts = itemdb.texts.remove(&Language::English).unwrap();

            AmbientData {
                items: Rc::new(itemdb.items),
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
