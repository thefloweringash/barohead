use std::rc::Rc;

use yew::prelude::*;
use yew_autocomplete::view::RenderHtml;
use yew_autocomplete::{view::Bulma, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;
use yew_router::prelude::*;

use crate::db::{SearchResult, DB};
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
    let db = use_context::<Rc<DB>>().unwrap();
    let mut peekable = search_result.indices.iter().peekable();

    // TODO: this is _really_ slow, you can feel the difference. It should use chunks.
    let item = db.get_item(search_result.item_ref);
    let description = db.item_translations.get_name(search_result.item_ref);
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
    let db = use_context::<Rc<DB>>().unwrap();
    let navigator = use_navigator().unwrap();

    let navigate_to_item = {
        let db = db.clone();
        Callback::from(move |items: Vec<SearchResult>| {
            let item_ref = &items.first().unwrap().item_ref;
            let item = db.get_item(*item_ref);
            navigator.push(&Route::Item {
                id: item.id.clone(),
            })
        })
    };

    let resolve_items: ItemResolver<SearchResult> = {
        FnProp::from(move |guess: String| -> ItemResolverResult<SearchResult> {
            let names = db.search(guess.as_str());
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
