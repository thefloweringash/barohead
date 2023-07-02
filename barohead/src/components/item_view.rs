use std::rc::Rc;

use yew::prelude::*;

use crate::{
    components::{ShowDeconstruct, ShowFabricate, ShowProcess},
    db::{DeconstructRef, FabricateRef, ItemRef, DB},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_ref: ItemRef,
}

#[function_component(ItemView)]
pub fn item_view(Props { item_ref }: &Props) -> Html {
    let db = use_context::<Rc<DB>>().unwrap();

    let item = db.get_item(*item_ref);
    let name = db.translations.get_name(item);

    let fabricates = item
        .fabricate
        .iter()
        .enumerate()
        .map(|(idx, _fabricate)| {
            let fabricate_ref = FabricateRef {
                item_ref: *item_ref,
                idx,
            };
            html! {
                <ShowFabricate self_ref={*item_ref} {fabricate_ref} />
            }
        })
        .collect::<Vec<_>>();

    let deconstructs = item
        .deconstruct
        .iter()
        .enumerate()
        .map(|(idx, _deconstruct)| {
            let deconstruct_ref = DeconstructRef {
                item_ref: *item_ref,
                idx,
            };
            html! {
                <ShowDeconstruct self_ref={*item_ref} {deconstruct_ref} />
            }
        })
        .collect::<Vec<_>>();

    let used_by = db.get_used_by(*item_ref).map(|used_by| {
        used_by
            .iter()
            .map(|process_ref| {
                let process_ref = process_ref.clone();
                html! {
                    <ShowProcess self_ref={*item_ref} {process_ref} />
                }
            })
            .collect::<Vec<_>>()
    });

    let produced_by = db.get_produced_by(*item_ref).map(|produced_by| {
        produced_by
            .iter()
            .map(|process_ref| {
                let process_ref = process_ref.clone();
                html! {
                    <ShowProcess self_ref={*item_ref} {process_ref} />
                }
            })
            .collect::<Vec<_>>()
    });

    html! {
        <div class="container">
            <h1>{name}</h1>
            <div class="panel">
                <div class="panel-heading">{"Details"}</div>
                <div class="panel-block">
                    <dl>
                        <dt>{"Id"}</dt>
                        <dd>{&item.id}</dd>
                    </dl>
                </div>
            </div>
            <div class="panel">
                <div class="panel-heading">{format!("Fabricated By ({})", fabricates.len())}</div>
                {fabricates}
            </div>
            <div class="panel">
                <div class="panel-heading">{format!("Deconstructs Into ({})", deconstructs.len())}</div>
                {deconstructs}
            </div>
            <div class="panel">
                <div class="panel-heading">{format!("Used By ({})", used_by.as_ref().map(|ub|ub.len()).unwrap_or(0))}</div>
                if used_by.is_some() {
                    {used_by.unwrap()}
                }
            </div>
            <div class="panel">
                <div class="panel-heading">{format!("Produced By ({})", produced_by.as_ref().map(|ub|ub.len()).unwrap_or(0))}</div>
                if produced_by.is_some() {
                    {produced_by.unwrap()}
                }
            </div>
            <div class="panel">
                <div class="panel-heading">{"Debug"}</div>
                <details class="panel-block">
                   <summary>{"Raw data"}</summary>

                    <pre>
                        {format!("{:#?}", item)}
                    </pre>
                </details>
            </div>
        </div>
    }
}
