use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use barohead_data::items::*;

use crate::{
    components::{ItemView, Nav},
    data::AmbientData,
    routes::Route,
};

#[derive(Properties, PartialEq)]
struct ItemPageProps {
    id: String,
}

#[function_component(ItemPage)]
fn item_page(ItemPageProps { id }: &ItemPageProps) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let item = ambient_data.get_item(id).unwrap();
    html! {
        <>
            <Nav />
            <ItemView {item} />
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Item { id } => html! {
           <ItemPage {id} />
        },
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! { <Nav /> }
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
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<Rc<AmbientData>>>
        </>
    }
}
