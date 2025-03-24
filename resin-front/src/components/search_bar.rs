use crate::router::ResinRoute;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::provider::ListingDataStore;

#[function_component(ListingsSearchBar)]
pub fn listings_search_bar() -> Html {
    let listing_ctx = use_context::<ListingDataStore>().expect("No context found!");

    let search_term = use_state(String::new);
    let filtered_listings = use_state(std::vec::Vec::new);
    let is_open = use_state(|| false);
    let node_ref = use_node_ref();

    let node_clone = node_ref.clone();
    let open_handle = is_open.setter();
    use_effect_with((), move |()| {
        let closure = web_sys::wasm_bindgen::closure::Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                if let Some(node) = node_clone.get() {
                    if !node.contains(Some(
                        &event.target().unwrap().unchecked_into::<web_sys::Node>(),
                    )) {
                        open_handle.set(false);
                    }
                }
            },
        ) as Box<dyn FnMut(_)>)
        .into_js_value();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        move || {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .remove_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    let filter_setter = filtered_listings.setter();
    let listings = listing_ctx.listings();
    use_effect_with(search_term.clone(), move |query| {
        if query.trim() == "" {
            filter_setter.set(vec![]);
        } else {
            let filtered = listings
                .iter()
                .filter(|(_id, listing)| {
                    listing.title.to_lowercase().contains(&query.to_lowercase())
                        // TODO
                        // cuando nos paguen
                        // || listing
                        //     .description
                        //     .to_lowercase()
                        //     .contains(&query.to_lowercase())
                        // || listing
                        //     .position
                        //     .full_address
                        //     .to_lowercase()
                        //     .contains(&query.to_lowercase())
                })
                .cloned()
                .collect::<Vec<_>>();
            filter_setter.set(filtered);
        }
        || {}
    });

    let oninput = {
        let search_term = search_term.clone();
        let is_open = is_open.clone();
        Callback::from(move |event: InputEvent| {
            search_term.set(event.target().unwrap().unchecked_into::<HtmlInputElement>().value());
            is_open.set(true);
        })
    };

    let onfocus = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(true);
        })
    };

    html! {
        <div class="flex-1 relative" ref={node_ref}>
          <div class="relative">
            <img src="/public/Search.svg" class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Search..."
              class="pl-10 pr-4 w-full p-3 rounded-xl bg-black text-white placeholder:text-white focus:outline-none focus:ring-2 focus:ring-bitcoin-orange focus:border-bitcoin-orange"
              value={(*search_term).clone()}
              {oninput}
              {onfocus}
            />
          </div>

          {if *is_open && filtered_listings.len() > 0 {
            html! {
                <div class="absolute z-10 mt-1 w-full rounded-xl border bg-black shadow-lg">
                  <ul class="max-h-60 overflow-auto py-1">
                    {for filtered_listings.iter().map(|(listing_id, listing)| {
                        let start_index = listing.title.to_lowercase().find(&*search_term.to_lowercase()).unwrap_or(0);
                        html! {
                            <Link<ResinRoute>
                                to={ResinRoute::Listing { listing_id: listing_id.clone() }}
                                >
                            <li
                                key={listing_id.to_string()}
                                class="cursor-pointer px-4 py-2 hover:bg-gray-800 text-sm font-semibold text-[#9CA3AF]"
                                >
                                {if start_index > 0 {
                                    html! {
                                        <div class="flex items-center">
                                        <img src="/public/Search.svg" class="size-3 mr-2" />
                                        {&listing.title[0..start_index]}
                                        <span class="text-bitcoin-orange font-black">{&listing.title[start_index..start_index + search_term.len()]}</span>
                                        {&listing.title[start_index + search_term.len()..]}
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <span >{&listing.title}</span>
                                    }
                                }}
                            </li>
                            </Link<ResinRoute>>
                        }})}
                  </ul>
                </div>
            }
          } else {
            html! {}}
          }
        </div>
    }
}
