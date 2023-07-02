use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

use crate::components::ItemSearch;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <Link<Route> to={Route::Home} classes="navbar-item">
                    {"BAROHEAD"}
                </Link<Route>>
                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <div class="navbar-item">
                        <ItemSearch />
                    </div>
                </div>
            </div>
        </nav>

    }
}
