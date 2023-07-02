use yew::prelude::*;

use crate::{
    components::{ShowDeconstruct, ShowFabricate},
    data::{ItemRef, ProcessRef},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub self_ref: ItemRef,
    pub process_ref: ProcessRef,
}

#[function_component(ShowProcess)]
pub fn show_process(
    Props {
        self_ref,
        process_ref,
    }: &Props,
) -> Html {
    match process_ref {
        ProcessRef::Fabricate(fabricate_ref) => {
            html! { <ShowFabricate self_ref={*self_ref} fabricate_ref={fabricate_ref.clone()} /> }
        }
        ProcessRef::Deconstruct(deconstruct_ref) => {
            html! { <ShowDeconstruct self_ref={*self_ref} deconstruct_ref={deconstruct_ref.clone()} /> }
        }
    }
}
