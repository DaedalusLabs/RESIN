use yew::prelude::*;

use crate::contexts::{provider::ListingDataStore, user_data::ResinUserStore};

#[function_component(FavoritesPage)]
pub fn favorites_page() -> Html {
    let user_ctx = use_context::<ResinUserStore>()
        .expect("FavoritesPage must be a child of ResinUserProvider");
    let listing_data = use_context::<ListingDataStore>()
        .expect("FavoritesPage must be a child of ResinUserProvider");
    let favorites = user_ctx.favorites();
    let favorite_listings = listing_data
        .listings()
        .iter()
        .cloned()
        .filter_map(|(listing_id, listing)| {
            if favorites.as_ref()?.favorite_listings.contains(&listing_id) {
                Some(html! {
                    <FavoritePreview {listing_id} {listing} />
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let inner_html = if favorites.is_some() {
        html! {
            <>
            {favorite_listings}
            </>
        }
    } else {
        html! {
            <div class="flex flex-col gap-4 p-4 border border-1 border-gray-300 items-center grow-0 h-fit">
                <h2 class="text-2xl font-black text-nowrap truncate">
                    {"No favorites found"}
                </h2>
                <Link<ResinRoute> to={ResinRoute::Search}>
                    <button class="bg-bitcoin-orange text-white rounded-lg py-2.5 px-5 font-medium">
                        {"Find Listings"}
                    </button>
                </Link<ResinRoute>>
            </div>
        }
    };

    html! {
        <div class="h-full w-full flex flex-col pt-16 gap-8 overflow-hidden">
            <h2 class="text-2xl leading-tight font-extrabold px-7">
            {"Favorites"}</h2>
            <div class="grid grid-cols-1 gap-4 h-full px-7 overflow-y-auto place-content-start">
                {inner_html}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FavoritePreviewProps {
    pub listing_id: String,
    pub listing: resin_core::NostrListing,
}
use crate::router::ResinRoute;
use yew_router::components::Link;

#[function_component(FavoritePreview)]
pub fn favorite_preview(props: &FavoritePreviewProps) -> Html {
    let FavoritePreviewProps {
        listing_id,
        listing,
    } = props;
    html! {
        <Link<ResinRoute> to={ResinRoute::Listing { listing_id: listing_id.clone() }}>
        <div class="flex flex-row gap-4 p-4 border border-1 border-gray-300 items-center grow-0 h-fit">
            <div class="aspect-square w-24 h-24 bg-gray-300 rounded-xl overflow-clip">
                <img class="object-cover w-full h-full" src={listing.images[0].clone()} />
            </div>
            <div class="flex flex-col gap-1 overflow-clip w-full">
                <h2 class="text-2xl font-black text-bitcoin-orange text-nowrap truncate">
                {&listing.title}</h2>
                <p class="text-sm leading-tight text-wrap line-clamp-2">{&listing.position.full_address}</p>
                <p class="font-bold text-nowrap truncate">{
                    format!("{} {}",
                    &listing.price.to_string(),
                    match listing.contract_type {
                        resin_core::ListingContractType::Rent => "per month",
                        _ => "",
                    })
                }</p>
            </div>
        </div>
        </Link<ResinRoute>>
    }
}
