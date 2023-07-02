use std::rc::Rc;

use yew::prelude::*;
use yew_autocomplete::view::RenderHtml;
use yew_autocomplete::{view::Bulma, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;
use yew_router::prelude::*;

use crate::data::{AmbientData, SearchResult};
use crate::routes::Route;

impl RenderHtml for SearchResult {
    fn render(&self) -> Html {
        html! { <ShowSearchResult search_result={self.clone()} /> }
    }
}

#[derive(Properties, PartialEq)]
struct Props {
    search_result: SearchResult,
}

#[function_component(ShowSearchResult)]
fn show_search_result(Props { search_result }: &Props) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let mut peekable = search_result.indices.iter().peekable();

    // TODO: this is _really_ slow, you can feel the difference. It should use chunks.
    let item = ambient_data.get_item(search_result.item_ref);
    let description = ambient_data.translations.get_name(item);
    let visible_match = description
        .char_indices()
        .map(|(idx, ch)| match peekable.peek() {
            Some(next_idx) if **next_idx == idx => {
                peekable.next();
                html! { <span key={idx} class="has-text-weight-bold">{ch}</span> }
            }
            _ => {
                html! { <span key={idx}>{ch}</span> }
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div>
            {visible_match}
            {" "}
            <span class="item-id">
                {"("}
                {&item.id}
                {")"}
            </span>
        </div>
    }
}

#[function_component(ItemSearch)]
pub fn item_search() -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let navigator = use_navigator().unwrap();

    let navigate_to_item = {
        let ambient_data = ambient_data.clone();
        Callback::from(move |items: Vec<SearchResult>| {
            let item_ref = &items.first().unwrap().item_ref;
            let item = ambient_data.get_item(*item_ref);
            navigator.push(&Route::Item {
                id: item.id.clone(),
            })
        })
    };

    let resolve_items: ItemResolver<SearchResult> = {
        FnProp::from(move |guess: String| -> ItemResolverResult<SearchResult> {
            let names = ambient_data.search(guess.as_str());
            Box::pin(async { Ok(names) })
        })
    };

    html! {
        <Autocomplete<SearchResult>
            {resolve_items}
            onchange={navigate_to_item}
            auto=true
        >
            <Bulma<SearchResult>
        />
        </Autocomplete<SearchResult>>
    }
}
