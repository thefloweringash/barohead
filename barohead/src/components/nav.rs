use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

use crate::components::ItemSearch;

#[function_component(Nav)]
pub fn nav() -> Html {
    let is_active = use_state(|| false);
    let toggle_active = {
        let is_active = is_active.clone();
        Callback::from(move |_| is_active.set(!*is_active))
    };

    let aria_expanded = if *is_active { "true" } else { "false" };
    html! {
        <nav class="navbar is-fixed-top" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <Link<Route> to={Route::Home} classes="navbar-item">
                    {"BAROHEAD"}
                </Link<Route>>
                <a
                    role="button"
                    class={classes!("navbar-burger", is_active.then(|| Some("is-active")))}
                    onclick={toggle_active}
                    aria-label="menu"
                    aria-expanded={aria_expanded}
                >
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
            </div>
            <div class={classes!("navbar-menu", is_active.then(|| Some("is-active")))}>
                <div class="navbar-start">
                    <div class="navbar-item">
                        <ItemSearch />
                    </div>
                </div>
                <div class="navbar-end">
                    <a class="navbar-item" href="https://github.com/thefloweringash/barohead">{"GitHub"}</a>
                </div>
            </div>
        </nav>

    }
}
