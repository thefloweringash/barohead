use std::rc::Rc;

use yew::prelude::*;
use yew_autocomplete::view::RenderHtml;
use yew_autocomplete::{view::Bulma, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;
use yew_router::prelude::*;

use barohead_data::items::*;

use crate::data::{AmbientData, DeconstructRef, FabricateRef, ProcessRef, SearchResult};

#[derive(Properties, PartialEq)]
struct ShowSearchResultProps {
    search_result: SearchResult,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/item/:id")]
    Item { id: String },
}

#[function_component(ShowSearchResult)]
fn show_search_result(ShowSearchResultProps { search_result }: &ShowSearchResultProps) -> Html {
    let mut peekable = search_result.indices.iter().peekable();

    // TODO: this is _really_ slow, you can feel the difference. It should use chunks.
    let visible_match = search_result
        .description
        .char_indices()
        .map(|(idx, ch)| match peekable.peek() {
            Some(next_idx) if **next_idx == idx => {
                peekable.next();
                html! { <span key={idx} class="has-text-weight-bold">{ch}</span> }
            }
            _ => {
                html! { <span key={idx}>{ch}</span> }
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div>
            {visible_match}
            {" "}
            <span class="item-id">
                {"("}
                {&search_result.item.id}
                {")"}
            </span>
        </div>
    }
}

impl RenderHtml for SearchResult {
    fn render(&self) -> Html {
        html! { <ShowSearchResult search_result={self.clone()} /> }
    }
}

#[function_component(ItemSearch)]
pub fn new_item_search() -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let navigator = use_navigator().unwrap();

    let navigate_to_item = Callback::from(move |items: Vec<SearchResult>| {
        let id = &items.first().unwrap().item.id;
        navigator.push(&Route::Item { id: id.clone() })
    });

    let resolve_items: ItemResolver<SearchResult> =
        FnProp::from(move |guess: String| -> ItemResolverResult<SearchResult> {
            let names = ambient_data.search(guess.as_str());
            Box::pin(async { Ok(names) })
        });

    html! {
        <Autocomplete<SearchResult>
            {resolve_items}
            onchange={navigate_to_item}
            auto=true
        >
            <Bulma<SearchResult>
        />
        </Autocomplete<SearchResult>>
    }
}

#[derive(Properties, PartialEq)]
struct ShowFabricateProps {
    self_id: String,
    fabricate_ref: FabricateRef,
}

#[function_component(ShowFabricate)]
fn show_fabricate(
    ShowFabricateProps {
        self_id,
        fabricate_ref,
    }: &ShowFabricateProps,
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
    html! {
        <div class="panel-block fabricate">
            <div class="required-items">{required_items}</div>
            <div class="production-arrow">{"->"}</div>
            <div class="produced-items">
                <ItemThumbnail {item} amount={fabricate.amount} condition={fabricate.out_condition} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ShowDeconstructProps {
    self_id: String,
    deconstruct_ref: DeconstructRef,
}

#[function_component(ShowDeconstruct)]
fn show_deconstruct(
    ShowDeconstructProps {
        self_id,
        deconstruct_ref,
    }: &ShowDeconstructProps,
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

#[derive(Properties, PartialEq)]
struct ShowProcessProps {
    self_id: String,
    process_ref: ProcessRef,
}

#[function_component(ShowProcess)]
fn show_process(
    ShowProcessProps {
        self_id,
        process_ref,
    }: &ShowProcessProps,
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

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: Rc<Item>,
}
#[function_component(ItemView)]
fn item_view(ItemProps { item }: &ItemProps) -> Html {
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

#[derive(Properties, PartialEq)]
struct ItemThumbnailProps {
    item: Rc<Item>,
    #[prop_or_default]
    amount: Option<i32>,
    #[prop_or_default]
    condition_range: Option<ConditionRange>,
    #[prop_or_default]
    condition: Option<f32>,
    #[prop_or_default]
    link: bool,
}
#[function_component(ItemThumbnail)]
fn item_thumbnail(
    ItemThumbnailProps {
        item,
        amount,
        condition,
        condition_range,
        link,
    }: &ItemThumbnailProps,
) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();

    // let id = item.id.as_str();

    // let name = ambient_data.translations.get_name(item);

    let body = html! {
        <>
            if amount.is_some() && amount.unwrap() != 1 {
              <span class="amount">{amount.unwrap()} {"x"}</span>
            }
            { " " }
            <span class="name">{ambient_data.translations.get_name(item)}</span>
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

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Vec<Rc<Item>>,
}
#[function_component(ItemList)]
fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
    items
        .iter()
        .map(|item| {
            html! {
                <ItemView key={item.id.as_str()} item={item.clone()} />
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct ItemPageProps {
    id: String,
}

#[function_component(ItemPage)]
fn item_page(ItemPageProps { id }: &ItemPageProps) -> Html {
    let ambient_data = use_context::<Rc<AmbientData>>().unwrap();
    let item = ambient_data.get_item(id).unwrap();
    html! {
        <>
            <Nav />
            <ItemView {item} />
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Item { id } => html! {
           <ItemPage {id} />
        },
    }
}

#[function_component(Nav)]
fn nav() -> Html {
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

#[function_component(Home)]
fn home() -> Html {
    html! { <Nav /> }
}

#[function_component(App)]
pub fn app() -> Html {
    // It might be easier to make this a lazy_static, but then we don't have the
    // option of having different databases for different versions.
    let ambient_data = use_memo(
        |_| {
            let items_bincode = std::include_bytes!("../recipes.bincode");
            let itemdb: ItemDB = bincode::deserialize(items_bincode).unwrap();
            AmbientData::from(itemdb)
        },
        (),
    );
    html! {
        <>
            <ContextProvider<Rc<AmbientData>> context={ambient_data}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<Rc<AmbientData>>>
        </>
    }
}
