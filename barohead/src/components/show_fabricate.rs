use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::ItemRef;

use crate::{
    components::ItemThumbnail,
    db,
    db::{FabricateRef, DB},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub self_ref: db::ItemRef,
    pub fabricate_ref: FabricateRef,
}

#[function_component(ShowFabricate)]
pub fn show_fabricate(
    Props {
        self_ref,
        fabricate_ref,
    }: &Props,
) -> Html {
    let db = use_context::<Rc<DB>>().unwrap();
    let fabricate = db.get_fabricate(fabricate_ref);
    let required_items = fabricate
        .required_items
        .iter()
        .map(|required_item| match &required_item.item {
            ItemRef::Id(input_item_id) => {
                let input_item_ref = db
                    .new_item_ref(input_item_id)
                    .expect("Fabricate required item");
                let is_self = &input_item_ref == self_ref;
                html! {
                    <ItemThumbnail
                        item_ref={input_item_ref}
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

    let output_is_self = fabricate_ref.item_ref == *self_ref;
    html! {
        <div class="panel-block fabricate">
            <div class="required-items">{required_items}</div>
            <div class="production-arrow">{"->"}</div>
            <div class="produced-items">
                <ItemThumbnail
                    item_ref={fabricate_ref.item_ref}
                    link={!output_is_self}
                    amount={fabricate.amount}
                    condition={fabricate.out_condition}
                />
            </div>
        </div>
    }
}
