use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::ItemRef;

use crate::{
    components::ItemThumbnail,
    data::{AmbientData, DeconstructRef},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub self_id: String,
    pub deconstruct_ref: DeconstructRef,
}

#[function_component(ShowDeconstruct)]
pub fn show_deconstruct(
    Props {
        self_id,
        deconstruct_ref,
    }: &Props,
) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let deconstruct = ambient_data.get_deconstruct(deconstruct_ref);
    let item = ambient_data
        .get_item(deconstruct_ref.item_id.as_ref())
        .unwrap();
    let showing_self = self_id == item.id.as_str();
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
                        link=true
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
            let is_self = &item.id == self_id;

            html! {
                <ItemThumbnail
                    {item}
                    link={!is_self}
                    amount={produced_item.amount}
                />
            }
        })
        .collect::<Vec<_>>();
    html! {
        <div class="panel-block deconstruct">
            <div class="required-items">
                <ItemThumbnail {item} link={!showing_self} />
                {required_items}
            </div>
            <div class="production-arrow">{"->"}</div>
            <div class="produced-items">{produced_items}</div>
        </div>
    }
}
