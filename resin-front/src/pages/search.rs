use crate::{
    components::{drawer::LeftDrawer, map_libre::MapLibreComponent, search_bar::ListingsSearchBar},
    contexts::provider::ListingDataStore,
    router::ResinRoute,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SearchSectionProps {
    pub listing_filter: UseStateHandle<Option<resin_core::NostrListing>>,
    pub show_map: UseStateHandle<bool>,
}

#[function_component(SearchPage)]
pub fn search_page() -> Html {
    let listing_filter = use_state(|| None::<resin_core::NostrListing>);
    let show_map = use_state(|| false);
    html! {
        <div class="flex flex-col h-full overflow-hidden pt-11 gap-2">
            <div class="flex-none">
                <SearchSection  listing_filter={listing_filter.clone()} show_map={show_map.clone()} />
            </div>
            <div class="flex-1 overflow-hidden">
            {if *show_map {
                    html! {
                        <div class="flex-1">
                            <MapLibreComponent />
                        </div>
                    }
                } else {
                    html! {
                        <NostrListingList {listing_filter} {show_map} />
                    }
                }}
            </div>
        </div>
    }
}

const FILTER_PROPERTY_TYPES: [resin_core::PropertyType; 8] = [
    resin_core::PropertyType::House,
    resin_core::PropertyType::Apartment,
    resin_core::PropertyType::Condo,
    resin_core::PropertyType::Townhouse,
    resin_core::PropertyType::Villa,
    resin_core::PropertyType::Land,
    resin_core::PropertyType::Commercial,
    resin_core::PropertyType::Other,
];
const FILTER_CONTRACT_TYPES: [resin_core::ListingContractType; 3] = [
    resin_core::ListingContractType::Sale,
    resin_core::ListingContractType::Rent,
    resin_core::ListingContractType::RentToOwn,
];
const FILTER_AVAILABILITY: [resin_core::ListingAvailability; 3] = [
    resin_core::ListingAvailability::LongTerm,
    resin_core::ListingAvailability::ShortTerm,
    resin_core::ListingAvailability::PerDirect,
];
const FILTER_AMENITIES: [resin_core::ListingAmenity; 16] = [
    resin_core::ListingAmenity::AirConditioning,
    resin_core::ListingAmenity::Balcony,
    resin_core::ListingAmenity::Elevator,
    resin_core::ListingAmenity::Fireplace,
    resin_core::ListingAmenity::Furnished,
    resin_core::ListingAmenity::Garden,
    resin_core::ListingAmenity::Gym,
    resin_core::ListingAmenity::Parking,
    resin_core::ListingAmenity::Pool,
    resin_core::ListingAmenity::SecuritySystem,
    resin_core::ListingAmenity::SolarPanels,
    resin_core::ListingAmenity::WheelchairAccessible,
    resin_core::ListingAmenity::Waterfront,
    resin_core::ListingAmenity::WaterHeater,
    resin_core::ListingAmenity::PetFriendly,
    resin_core::ListingAmenity::Terrace,
];
#[function_component(SearchSection)]
pub fn search_bar(props: &SearchSectionProps) -> Html {
    let filtered_listing = props.listing_filter.clone();
    let filter_is_open = use_state(|| false);
    let filter_onclick = {
        let filter_is_open = filter_is_open.clone();
        Callback::from(move |_| filter_is_open.set(!*filter_is_open))
    };

    let on_price_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let price = target.value().parse::<f64>().unwrap_or(0.0);
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.price.price = price;
            filtered_listing.set(Some(listing));
        })
    };

    let on_size_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let size = target.value().parse::<f64>().unwrap_or(0.0);
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.size.size = size;
            filtered_listing.set(Some(listing));
        })
    };

    let on_bedrooms_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let bedrooms = target.value().parse::<u8>().unwrap_or(0);
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.bedrooms = bedrooms;
            filtered_listing.set(Some(listing));
        })
    };

    let on_bathrooms_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let bathrooms = target.value().parse::<u8>().unwrap_or(0);
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.bathrooms = bathrooms;
            filtered_listing.set(Some(listing));
        })
    };

    let on_property_type_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let property_type = target.value();
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.property_type = property_type
                .parse()
                .unwrap_or(resin_core::PropertyType::Other);
            filtered_listing.set(Some(listing));
        })
    };

    let on_contract_type_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let contract_type = target.value();
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.contract_type = contract_type
                .parse()
                .unwrap_or(resin_core::ListingContractType::Rent);
            filtered_listing.set(Some(listing));
        })
    };

    let on_availability_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let availability = target.value();
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            listing.availability = availability
                .parse()
                .unwrap_or(resin_core::ListingAvailability::PerDirect);
            filtered_listing.set(Some(listing));
        })
    };

    let on_amenities_change = {
        let filtered_listing = filtered_listing.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .map(wasm_bindgen::JsCast::unchecked_into::<HtmlInputElement>)
                .expect("target is not an input element");
            let amenity = target.value();
            let mut listing = (*filtered_listing)
                .clone()
                .unwrap_or_default();
            let amenity: resin_core::ListingAmenity = amenity
                .parse()
                .unwrap_or(resin_core::ListingAmenity::AirConditioning);
            if listing.amenities.contains(&amenity) {
                listing.amenities.retain(|a| a != &amenity);
            } else {
                listing.amenities.push(amenity);
            }
            filtered_listing.set(Some(listing));
        })
    };

    let show_map_click = {
        let show_map = props.show_map.clone();
        Callback::from(move |_| show_map.set(!*show_map))
    };

    let show_map_icon = if *props.show_map {
        html! {
            <img src="/public/List.svg" class="size-6" />
        }
    } else {
        html! {
            <img src="/public/MapPin.svg" class="size-6" />
        }
    };

    let filtered_listing = (*filtered_listing)
        .clone()
        .unwrap_or_default();
    html! {
        <>
        <div class="flex flex-row w-full justify-between gap-x-2 px-4 items-center">
            <button onclick={show_map_click} 
                class="rounded-xl bg-black text-white p-3">
                {show_map_icon}
            </button>
            <ListingsSearchBar />
            <button onclick={filter_onclick} class="rounded-xl bg-black text-white p-3">
                <img src="/public/Sliders.svg" class="size-6 font-bold" />
            </button>
        </div>

        <LeftDrawer is_open={filter_is_open}>
            <div class="flex flex-col p-6 bg-white h-full overflow-y-auto">
                <h2 class="text-2xl font-bold mb-6 text-gray-800">{"Filters"}</h2>
                <img src="/public/BG_Hexagon_White.svg" class="fixed inset-0 opacity-10" />
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Price"}</h3>
                    <div class="mb-2 flex justify-between text-sm text-gray-500">
                        <span>{"$0"}</span>
                        <span>{"$2,000,000"}</span>
                    </div>
                    <input 
                        onchange={on_price_change} 
                        value={filtered_listing.price.price.to_string()}
                        type="range" 
                        min="0" 
                        max="2000000" 
                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                    />
                    <div class="mt-2 text-sm font-medium text-gray-600">
                        {"Selected: $"}{filtered_listing.price.price.to_string()}
                    </div>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Size"}</h3>
                    <div class="mb-2 flex justify-between text-sm text-gray-500">
                        <span>{"0 sq ft"}</span>
                        <span>{"300 sq ft"}</span>
                    </div>
                    <input 
                        onchange={on_size_change} 
                        value={filtered_listing.size.size.to_string()}
                        type="range" 
                        min="0" 
                        max="300" 
                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                    />
                    <div class="mt-2 text-sm font-medium text-gray-600">
                        {"Selected: "}{filtered_listing.size.size.to_string()}{" sq ft"}
                    </div>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Bedrooms"}</h3>
                    <div class="mb-2 flex justify-between text-sm text-gray-500">
                        <span>{"0"}</span>
                        <span>{"10"}</span>
                    </div>
                    <input 
                        onchange={on_bedrooms_change} 
                        value={filtered_listing.bedrooms.to_string()}
                        type="range" 
                        min="0" 
                        max="10" 
                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                    />
                    <div class="mt-2 text-sm font-medium text-gray-600">
                        {"Selected: "}{filtered_listing.bedrooms.to_string()}
                    </div>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Bathrooms"}</h3>
                    <div class="mb-2 flex justify-between text-sm text-gray-500">
                        <span>{"0"}</span>
                        <span>{"10"}</span>
                    </div>
                    <input 
                        onchange={on_bathrooms_change} 
                        value={filtered_listing.bathrooms.to_string()}
                        type="range" 
                        min="0" 
                        max="10" 
                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                    />
                    <div class="mt-2 text-sm font-medium text-gray-600">
                        {"Selected: "}{filtered_listing.bathrooms.to_string()}
                    </div>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Property Type"}</h3>
                    <select 
                        onchange={on_property_type_change}
                        class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-bitcoin-orange focus:border-bitcoin-orange"
                    >
                        <option>{"Property Type"}</option>
                        {for FILTER_PROPERTY_TYPES.iter().map(|property_type| {
                            html! {
                                <option>{property_type.to_string()}</option>
                            }
                        })}
                    </select>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Contract Type"}</h3>
                    <select 
                        onchange={on_contract_type_change}
                        class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-bitcoin-orange focus:border-bitcoin-orange"
                    >
                        <option>{"Contract Type"}</option>
                        {for FILTER_CONTRACT_TYPES.iter().map(|contract_type| {
                            html! {
                                <option>{contract_type.to_string()}</option>
                            }
                        })}
                        <option>{filtered_listing.contract_type.to_string()}</option>
                    </select>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Availability"}</h3>
                    <select 
                        onchange={on_availability_change}
                        class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-bitcoin-orange focus:border-bitcoin-orange"
                    >
                        <option>{"Availability"}</option>
                        {for FILTER_AVAILABILITY.iter().map(|availability| {
                            html! {
                                <option>{availability.to_string()}</option>
                            }
                        })}
                    </select>
                </div>
                
                <div class="mb-6">
                    <h3 class="text-lg font-medium mb-3 text-gray-700">{"Amenities"}</h3>
                    <div class="grid grid-cols-2 gap-3">
                        {for FILTER_AMENITIES.iter().map(|amenity| {
                            html! {
                                <label class="flex items-center space-x-2 cursor-pointer">
                                    <input 
                                        onchange={on_amenities_change.clone()} 
                                        value={amenity.clone().to_string()}
                                        type="checkbox" 
                                        class="w-4 h-4 text-bitcoin-orange border-gray-300 rounded focus:ring-bitcoin-orange"
                                    />
                                    <span class="text-sm text-gray-600">{amenity.to_string()}</span>
                                </label>
                            }
                        })}
                    </div>
                </div>
            </div>
        </LeftDrawer>
        </>
    }
}

#[function_component(NostrListingList)]
pub fn nostr_listing_list(props: &SearchSectionProps) -> Html {
    let listings_ctx = use_context::<ListingDataStore>()
        .expect("NostrListingList must be a child of ListingDataProvider");
    let filter = (*props.listing_filter).clone();

    html! {
        <div class="grid grid-cols-1 gap-4 overflow-y-auto h-full px-4">
        {for listings_ctx.listings().iter().filter(|&(_,listing)| {
            if filter.is_none() {
                return true;
            };
            let filter = filter.clone().unwrap();
            filter.price <= listing.price
                && filter.size <= listing.size
                && filter.bedrooms <= listing.bedrooms
                && filter.bathrooms <= listing.bathrooms
                && filter.amenities.iter().all(|amenity| listing.amenities.contains(amenity))

            }).cloned().map(|(listing_id, listing)| {
            html! {
                <NostrListingCard {listing_id} {listing} />
            }
        })}
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NostrListingCardProps {
    pub listing_id: String,
    pub listing: resin_core::NostrListing,
}

#[function_component(NostrListingCard)]
pub fn nostr_listing_card(props: &NostrListingCardProps) -> Html {
    let NostrListingCardProps {
        listing_id,
        listing,
    } = props.clone();

    let image_index = use_state(|| 0);

    let onclick = {
        let image_index = image_index.clone();
        let listing_len = listing.images.len();
        Callback::from(move |_| {
            let new_index = (*image_index + 1) % listing_len;
            image_index.set(new_index);
        })
    };

    html! {
        <div class="aspect-square gap-4 rounded-xl flex flex-col overflow-clip">
            <div class="z-0 relative flex-1 bg-gray-200">
                <img class="object-cover absolute inset-0" src={listing.images[*image_index].clone()} />
                <div {onclick}
                    class="absolute top-4 right-2 bg-white text-bitcoin-orange rounded-full p-4">
                    <lucide_yew::Images class="w-6 h-6" />
                </div>
                <div class="absolute bottom-0 right-2 bg-white/70 rounded-full p-2 border border-2 border-bitcoin-orange backdrop-blur-md">
                    <p class="text-bitcoin-orange text-xs font-bold">{listing.contract_type.to_string()}</p>
                </div>
            </div>
            <div class="z-10 flex flex-col gap-2 p-4 bg-white">
                <p class="text-bitcoin-orange font-black text-2xl">
                    {&listing.title.clone()}
                </p>
                <p>{&listing.position.full_address}</p>
                <p class="font-bold">
                    {format!("{} {}",
                    &listing.price.to_string(),
                    match listing.contract_type {
                        resin_core::ListingContractType::Rent => "per month",
                        _ => "",
                    })}
                </p>
                <div class="w-full flex items-center flex-row justify-between">
                    <div class="w-full flex flex-row gap-5">
                        <div class="flex items-center gap-2">
                            <img src="/public/SquareMeters.svg" alt="SquareMeters Icon" class="size-4" />
                            <p>{listing.size.to_string()}</p>                    
                        </div>
                        <div class="flex items-center gap-2">
                            <img src="/public/SingleBed.svg" alt="Bed Icon" class="size-4" />
                            <p>{listing.bedrooms}{" beds"}</p>
                        </div>
                    </div>
                    <div class="w-1/3 mx-auto">
                        <yew_router::components::Link<ResinRoute> to={ResinRoute::Listing { listing_id }}>
                            <button class="bg-bitcoin-orange text-white rounded-lg py-2 px-5 flex justify-center items-center flex-row gap-3 w-full">
                                <p class="text-center">{"Details"}</p>
                                <img src="/public/RightArrow.svg" class="size-4" />
                            </button>
                        </yew_router::components::Link<ResinRoute>>
                    </div>
                </div>
            </div>
        </div>
    }
}
