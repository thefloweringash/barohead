use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::ItemRef;

use crate::{
    components::ItemThumbnail,
    data,
    data::{AmbientData, DeconstructRef},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub self_ref: data::ItemRef,
    pub deconstruct_ref: DeconstructRef,
}

#[function_component(ShowDeconstruct)]
pub fn show_deconstruct(
    Props {
        self_ref,
        deconstruct_ref,
    }: &Props,
) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let deconstruct = ambient_data.get_deconstruct(deconstruct_ref);
    let showing_self = deconstruct_ref.item_ref == *self_ref;
    let required_items = deconstruct
        .required_items
        .iter()
        .map(|required_item| match &required_item.item {
            ItemRef::Id(id) => {
                let item_ref = ambient_data
                    .new_item_ref(id)
                    .expect("Deconstruct Required item");
                html! {
                    <ItemThumbnail
                        {item_ref}
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
            let item_ref = ambient_data
                .new_item_ref(produced_item.id.as_str())
                .expect("Deconstruct Produced item");
            // TODO: The produced items are conditional based on input condition.
            let is_self = item_ref == *self_ref;

            html! {
                <ItemThumbnail
                    {item_ref}
                    link={!is_self}
                    amount={produced_item.amount}
                />
            }
        })
        .collect::<Vec<_>>();
    html! {
        <div class="panel-block deconstruct">
            <div class="required-items">
                <ItemThumbnail item_ref={deconstruct_ref.item_ref} link={!showing_self} />
                {required_items}
            </div>
            <div class="production-arrow">{"->"}</div>
            <div class="produced-items">{produced_items}</div>
        </div>
    }
}
