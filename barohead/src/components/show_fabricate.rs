use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::ItemRef;

use crate::{
    components::ItemThumbnail,
    data::{AmbientData, FabricateRef},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub self_id: String,
    pub fabricate_ref: FabricateRef,
}

#[function_component(ShowFabricate)]
pub fn show_fabricate(
    Props {
        self_id,
        fabricate_ref,
    }: &Props,
) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let fabricate = ambient_data.get_fabricate(fabricate_ref);
    let item = ambient_data
        .get_item(fabricate_ref.item_id.as_str())
        .unwrap();
    let required_items = fabricate
        .required_items
        .iter()
        .map(|required_item| match &required_item.item {
            ItemRef::Id(id) => {
                let input_item = ambient_data.get_item(id).expect("Fabricate Required item");
                let is_self = input_item.id == self_id.as_str();
                html! {
                    <ItemThumbnail
                        item={input_item}
                        link={!is_self}
                        amount={required_item.amount}
                        condition_range={required_item.condition.clone()}
                    />
                }
            }
            ItemRef::Tag(tag) => {
                html! {
                    <div>
                    {"(Tag) "}
                    if required_item.amount != 1 {
                        <span class="amount">{required_item.amount}</span>
                    }
                    <span class="name">{tag}</span>
                    if required_item.condition.is_some() {
                        <span class="condition">{format!("{:#?}", required_item.condition)}</span>
                    }
                    </div>
                }
            }
        })
        .collect::<Vec<_>>();

    let output_is_self = &item.id == self_id;
    html! {
        <div class="panel-block fabricate">
            <div class="required-items">{required_items}</div>
            <div class="production-arrow">{"->"}</div>
            <div class="produced-items">
                <ItemThumbnail
                    {item}
                    link={!output_is_self}
                    amount={fabricate.amount}
                    condition={fabricate.out_condition}
                />
            </div>
        </div>
    }
}
