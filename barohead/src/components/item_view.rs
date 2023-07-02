use std::rc::Rc;

use yew::prelude::*;

use barohead_data::items::Item;

use crate::{
    components::{ShowDeconstruct, ShowFabricate, ShowProcess},
    data::{AmbientData, DeconstructRef, FabricateRef},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Rc<Item>,
}

#[function_component(ItemView)]
pub fn item_view(Props { item }: &Props) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();

    let id = item.id.as_str();

    let name = ambient_data.translations.get_name(item);

    let fabricates = item
        .fabricate
        .iter()
        .enumerate()
        .map(|(idx, _fabricate)| {
            let fabricate_ref = FabricateRef {
                item_id: id.to_owned(),
                idx,
            };
            html! {
                <ShowFabricate self_id={id.to_owned()} {fabricate_ref} />
            }
        })
        .collect::<Vec<_>>();

    let deconstructs = item
        .deconstruct
        .iter()
        .enumerate()
        .map(|(idx, _deconstruct)| {
            let deconstruct_ref = DeconstructRef {
                item_id: id.to_owned(),
                idx,
            };
            html! {
                <ShowDeconstruct self_id={id.to_owned()} {deconstruct_ref} />
            }
        })
        .collect::<Vec<_>>();

    let used_by = ambient_data.get_used_by(id).map(|used_by| {
        used_by
            .iter()
            .map(|process_ref| {
                let self_id = item.id.clone();
                let process_ref = process_ref.clone();
                html! {
                    <ShowProcess {self_id} {process_ref} />
                }
            })
            .collect::<Vec<_>>()
    });

    let produced_by = ambient_data.get_produced_by(id).map(|produced_by| {
        produced_by
            .iter()
            .map(|process_ref| {
                let self_id = item.id.clone();
                let process_ref = process_ref.clone();
                html! {
                    <ShowProcess {self_id} {process_ref} />
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
                        <dd>{id}</dd>
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
