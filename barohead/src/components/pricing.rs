use std::rc::Rc;

use barohead_data::items::{Price, StoreIdentifier};
use yew::prelude::*;

use crate::db::{ItemRef, DB, INTERESTING_MERCHANTS};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_ref: ItemRef,
}

struct StoreSummary {
    pub sell: Option<i32>,
    pub buy: i32,
}

impl StoreSummary {
    fn for_store(price: &Price, store: StoreIdentifier) -> Self {
        let matching_modifier = price.modifiers.get(&store);

        let sold = if Self::is_specialist_merchant(store) {
            matching_modifier
                .map(|m| m.sold.unwrap_or(price.sold))
                .unwrap_or(false)
        } else {
            matching_modifier.and_then(|m| m.sold).unwrap_or(price.sold)
        };

        let multiplier = matching_modifier.and_then(|m| m.multiplier);

        let sell_price: f32 = multiplier
            .map(|mul| mul * price.baseprice as f32)
            .unwrap_or(price.baseprice as f32);

        let buy_price: f32 = sell_price * 0.3;

        Self {
            buy: buy_price as i32,
            sell: sold.then_some(sell_price as i32),
        }
    }

    fn is_specialist_merchant(store: StoreIdentifier) -> bool {
        matches!(
            store,
            StoreIdentifier::MerchantMedical
                | StoreIdentifier::MerchantEngineering
                | StoreIdentifier::MerchantArmory
                | StoreIdentifier::MerchantClown
                | StoreIdentifier::MerchantHusk
        )
    }
}

fn format_price(x: i32) -> Html {
    html! {<> <strong>{x}</strong>{ " mk"} </>}
}

fn format_optional_price(x: Option<i32>) -> Html {
    x.map(format_price).unwrap_or(html! { "N/A" })
}

#[function_component(PricingView)]
pub fn pricing_view(Props { item_ref }: &Props) -> Html {
    let db = use_context::<Rc<DB>>().unwrap();

    let item = db.get_item(*item_ref);

    if let Some(price) = item.price.as_ref() {
        let rows = INTERESTING_MERCHANTS
            .iter()
            .map(|store| {
                let store_name = db.store_translations.get_name(store);
                let pricing = StoreSummary::for_store(price, *store);
                html! {
                    <tr>
                        <th>{store_name}</th>
                        <td>{format_optional_price(pricing.sell)}</td>
                        <td>{format_price(pricing.buy)}</td>
                    </tr>
                }
            })
            .collect::<Vec<_>>();

        html! {
            <table class="table">
              <thead>
                  <tr>
                      <th>{"Merchant"}</th>
                      <th>{"Buy"}</th>
                      <th>{"Sell"}</th>
                  </tr>
              </thead>
              <tbody>
                  {rows}
              </tbody>
            </table>
        }
    } else {
        html! {
            "No pricing data"
        }
    }
}
