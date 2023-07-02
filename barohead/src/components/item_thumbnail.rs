use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use barohead_data::items::ConditionRange;

use crate::db::{ItemRef, DB};
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_ref: ItemRef,
    #[prop_or_default]
    pub amount: Option<i32>,
    #[prop_or_default]
    pub condition_range: Option<ConditionRange>,
    #[prop_or_default]
    pub condition: Option<f32>,
    #[prop_or_default]
    pub link: bool,
}

#[function_component(ItemThumbnail)]
pub fn item_thumbnail(
    Props {
        item_ref,
        amount,
        condition,
        condition_range,
        link,
    }: &Props,
) -> Html {
    let db = use_context::<Rc<DB>>().unwrap();

    let item = db.get_item(*item_ref);

    let body = html! {
        <>
            if amount.is_some() && amount.unwrap() != 1 {
              <span class="amount">{amount.unwrap()} {"x"}</span>
            }
            { " " }
            <span class="name">{db.translations.get_name(item)}</span>
            if condition_range.is_some() {
                <span class="condition-range">{format!("{:#?}", condition_range)}</span>
            }
            if condition.is_some() && condition.unwrap() != 1.0 {
                <span class="conditione">{format!("{:#?}", condition)}</span>
            }
        </>
    };

    html! {
        if *link {
            <Link<Route> to={Route::Item{ id: item.id.clone() }} classes="item-thumbnail">
                {body}
            </Link<Route>>
        } else {
            <div class="item-thumbnail">
                {body}
            </div>
        }
    }
}
