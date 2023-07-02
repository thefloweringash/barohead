use yew::prelude::*;

use crate::{
    components::{ShowDeconstruct, ShowFabricate},
    data::ProcessRef,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub self_id: String,
    pub process_ref: ProcessRef,
}

#[function_component(ShowProcess)]
pub fn show_process(
    Props {
        self_id,
        process_ref,
    }: &Props,
) -> Html {
    let self_id = self_id.clone();
    match process_ref {
        ProcessRef::Fabricate(fabricate_ref) => {
            html! { <ShowFabricate {self_id} fabricate_ref={fabricate_ref.clone()} /> }
        }
        ProcessRef::Deconstruct(deconstruct_ref) => {
            html! { <ShowDeconstruct {self_id} deconstruct_ref={deconstruct_ref.clone()} /> }
        }
    }
}
