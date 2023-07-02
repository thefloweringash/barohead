use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use barohead_data::items::*;

use crate::{
    components::{ItemView, Nav},
    db::DB,
    routes::Route,
};

#[derive(Properties, PartialEq)]
struct ItemPageProps {
    id: AttrValue,
}

#[function_component(ItemPage)]
fn item_page(ItemPageProps { id }: &ItemPageProps) -> Html {
    let db = use_context::<Rc<DB>>().unwrap();
    let item_ref = db.new_item_ref(id).expect("Loading item data");
    html! {
        <>
            <Nav />
            <ItemView item_ref={item_ref} />
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Item { id } => {
            html! {
               <ItemPage id={id} />
            }
        }
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
    let db = use_memo(
        |_| {
            let items_bincode = std::include_bytes!("../recipes.bincode");
            let item_data: ItemDB = bincode::deserialize(items_bincode).unwrap();
            DB::from(item_data)
        },
        (),
    );
    html! {
        <>
            <ContextProvider<Rc<DB>> context={db}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<Rc<DB>>>
        </>
    }
}
