use yew::prelude::*;
use crate::ListingDataStore;


#[function_component(NostrListingList)]
pub fn nostr_listing_list() -> Html {
    let listings_ctx = use_context::<ListingDataStore>()
        .expect("NostrListingList must be a child of ListingDataProvider");

    html! {
        {for listings_ctx.listings().iter().map(|listing| {
            html! {
                <div>
                    <p>{listing.title.clone()}</p>
                    <p>{listing.price.to_string()}</p>
                </div>
            }
        })}
    }
}
