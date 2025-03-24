use nostr_minions::{key_manager::NostrIdStore, relay_pool::NostrPoolStore};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{map_libre::ListingMapPreview, modal::Modal},
    contexts::{
        chat_provider::{ResinChatAction, ResinChatStore},
        provider::ListingDataStore,
        user_data::ResinUserStore,
    },
    router::ResinRoute,
};

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct ListingDetailProps {
    pub listing_id: String,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListingProps {
    pub listing_id: String,
    pub listing: resin_core::NostrListing,
}

#[function_component(ListingDetailPage)]
pub fn listing_detail_page(props: &ListingDetailProps) -> Html {
    let ListingDetailProps { listing_id } = props;
    let listings_ctx = use_context::<ListingDataStore>()
        .expect("ListingDetailPage must be a child of ListingDataProvider");
    let find_listing = listings_ctx
        .find_listing(listing_id)
        .expect("Listing not found");

    html! {
        <div class="flex flex-col flex-1 overflow-hidden gap-1 h-full">
            <ListingMainPicture listing_id={listing_id.clone()} listing={find_listing.clone()} />
            <ListingDetailSection listing_id={listing_id.clone()} listing={find_listing.clone()} />
            <ContactAgentButton listing_id={listing_id.clone()} listing={find_listing.clone()} />
        </div>
    }
}

#[function_component(ContactAgentButton)]
pub fn contact_agent_button(props: &ListingProps) -> Html {
    let is_open = use_state(|| false);
    let relay_ctx = use_context::<NostrPoolStore>()
        .expect("ContactAgentButton must be a child of NostrPoolProvider");
    let id_ctx = use_context::<NostrIdStore>()
        .expect("ContactAgentButton must be a child of NostrIdProvider");
    let chat_ctx = use_context::<ResinChatStore>()
        .expect("ContactAgentButton must be a child of ResinUserProvider");

    let navigator = use_navigator().expect("No navigator found");

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };
    let ListingProps {
        listing_id: _,
        listing,
    } = props.clone();

    let send_message = {
        let listing = listing.clone();
        let chat_ctx = chat_ctx;
        let navigator = navigator;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let listing = listing.clone();
            let sender = relay_ctx.send_note.clone();
            let identity = id_ctx.clone();
            let chat_ctx = chat_ctx.clone();
            let navigator = navigator.clone();
            yew::platform::spawn_local(async move {
                let message = format!(
                    "I want to request a tour to view {}",
                    listing.position.full_address
                );
                let Some(identity) = identity.get_identity().cloned() else {
                    navigator.push(&ResinRoute::Login);
                    return;
                };
                let Ok(new_chat) = chat_ctx
                    .new_resin_dm(
                        message,
                        listing.title,
                        "9079abf2786a4e0497873cf120290ec51cd57990ce04cc16a26f89f4f3f45128"
                            .to_string(),
                        identity,
                    )
                    .await
                else {
                    gloo::console::error!("Failed to create new message");
                    return;
                };
                sender.emit(new_chat.0);
                chat_ctx.dispatch(ResinChatAction::NewUserMessage(new_chat.1));
                navigator.push(&ResinRoute::Messages);
            });
        })
    };
    html! {
        <>
        <button {onclick} class="absolute z-20 bottom-4 right-4 bg-bitcoin-orange text-white w-fit p-2 flex flex-row gap-1 rounded-xl">
            <p>{"Contact Agent"}</p>
            <lucide_yew::ArrowRight class="w-6 h-6" />
        </button>
        <Modal {is_open}>
            <form class="bg-white flex flex-col rounded-2xl h-fit">
                <h2 class="font-bold max-w-2/3 text-wrap p-4">{format!("Send a tour request for {}", listing.title)}</h2>
                <div class="flex flex-row gap-2 p-4 w-full items-center">
                    <div class="flex flex-row gap-2 p-4 w-full border border-gray-200 border-2 rounded-lg items-center">
                        <p class="p-2 border-none text-sm leading-tight">{format!("I want to request a tour to view {}", listing.position.full_address)} </p>
                        <input onclick={send_message} type="submit" class="bg-bitcoin-orange text-white px-5 py-2 rounded-lg" value="Send" />
                    </div>
                </div>
            </form>
        </Modal>
        </>
    }
}

#[function_component(ListingDetailSection)]
pub fn listing_detail_section(props: &ListingProps) -> Html {
    let ListingProps {
        listing_id: _,
        listing,
    } = props.clone();
    html! {
            <div class="grid grid-cols-1 gap-8 overflow-y-auto px-7 py-3 h-[calc(100%-min(50vh,400px))] max-h-[50vh]">
                <div class="flex flex-col gap-2">
                    <h2 class="text-2xl font-extrabold leading-tight">{&listing.title}</h2>
                    <p class="text-sm" >{&listing.position.full_address}</p>
                    <div class="flex items-end gap-3 pt-3 text-xs text-[#303031]">
                        <div class="flex items-center gap-1">
                            <img src="/public/SquareMeters.svg" class="size-4" />
                            <p >{listing.size.to_string()}</p>
                        </div>

                        <div class="flex items-center gap-1">
                            <img src="/public/LotSize.svg" class="size-4" />
                            <p >{listing.lot_size.clone().unwrap_or_default().to_string()}</p>
                        </div>

                        <div class="flex items-center gap-1">
                            <img src="/public/SingleBed.svg" class="size-4" />
                            <p >{format!("{} beds", listing.bedrooms)}</p>
                        </div>
                    </div>
                </div>
                <div class="space-y-1">
                    <h3 class="text-xs text-[#303031]">{"Property Price"}</h3>
                    <p class="pt-2 text-lg font-semibold">{listing.price.to_string()}</p>
                </div>
                <div>
                    <h3 class="font-bold">{"Summary"}</h3>
                    <p class="pt-2 text-sm">{&listing.description}</p>
                </div>
                <div>
                    <h3 class="font-bold">{"Key Features"}</h3>
                    <ul class="pt-2 flex flex-row flex-wrap gap-2">
                        {for listing.amenities.iter().map(|feature| {
                            html! {
                                <li class="bg-[#E6E6E7] rounded-2xl py-0.5 px-1.5 text-xs font-bold text-[#6C6D6E]">{feature.to_string()}</li>
                            }
                        })}
                    </ul>
                </div>
                <div>
                    <h3 class="font-bold">{"Additional Details"}</h3>
                    <div class="pt-2 flex flex-row ">
                        <p class="text-xs w-1/2" >{"Bathrooms"}</p>
                        <p class="bg-[#E6E6E7] rounded-2xl py-0.5 px-1.5 text-xs font-bold text-[#6C6D6E]">{listing.bathrooms.to_string()}</p>
                    </div>
                    <div class="pt-2 flex flex-row ">
                        <p class="text-xs w-1/2" >{"Property Type"}</p>
                        <p class="bg-[#E6E6E7] rounded-2xl py-0.5 px-1.5 text-xs font-bold text-[#6C6D6E]">{listing.property_type.to_string()}</p>
                    </div>
                    <div class="pt-2 flex flex-row ">
                        <p class="text-xs w-1/2" >{"Area"}</p>
                        <p class="bg-[#E6E6E7] rounded-2xl py-0.5 px-1.5 text-xs font-bold text-[#6C6D6E]">{listing.location.unwrap_or_default().to_string()}</p>
                    </div>
                    <div class="pt-2 flex flex-row">
                        <p class="text-xs w-1/2">{"Lot Size"}</p>
                        <p class="bg-[#E6E6E7] rounded-2xl py-0.5 px-1.5 text-xs font-bold text-[#6C6D6E]">{listing.lot_size.unwrap_or_default().to_string()}</p>
                    </div>
                </div>
                <div>
                    <h3 class="font-bold">{"Location"}</h3>
                    <div class="mt-2 aspect-square rounded-2xl bg-gray-200 overflow-clip">
                        <ListingMapPreview geolocation={listing.position.geopoint.clone()} />
                    </div>
                </div>
            </div>
    }
}

#[function_component(ListingMainPicture)]
pub fn listing_main_picture(props: &ListingProps) -> Html {
    let image_index = use_state(|| 0);
    let user_ctx = use_context::<ResinUserStore>()
        .expect("ListingMainPicture must be a child of ResinUserProvider");
    let id_ctx = use_context::<NostrIdStore>()
        .expect("ListingMainPicture must be a child of NostrIdProvider");
    let relay_ctx = use_context::<NostrPoolStore>()
        .expect("ListingMainPicture must be a child of NostrPoolProvider");
    let navigator = use_navigator().expect("No navigator found");
    let ListingProps {
        listing_id,
        listing,
    } = props.clone();
    let favorites = user_ctx.favorites().unwrap_or_default();
    let is_already_favorite = favorites.favorite_listings.contains(&listing_id);
    let on_favorite_click = {
        let sender = relay_ctx.send_note;
        Callback::from(move |_| {
            let mut new_favorites = favorites.clone();
            if is_already_favorite {
                new_favorites
                    .favorite_listings
                    .retain(|id| id != &listing_id);
            } else {
                new_favorites.favorite_listings.push(listing_id.clone());
            }
            let mut new_favorites_note: nostro2::note::NostrNote = new_favorites.into();
            let Some(pubkey) = id_ctx.get_pubkey() else {
                navigator.push(&ResinRoute::Login);
                return;
            };
            new_favorites_note.pubkey.clone_from(&pubkey);
            let id_clone = id_ctx.clone();
            let sender = sender.clone();
            yew::platform::spawn_local(async move {
                let Ok(mut giftwrapped_faves) =
                    id_clone.create_giftwrap(new_favorites_note, 10401).await
                else {
                    gloo::console::error!("Failed to create giftwrap");
                    return;
                };
                id_clone
                    .sign_encrypted_note(&mut giftwrapped_faves, pubkey)
                    .await
                    .expect("Failed to sign note");
                sender.emit(giftwrapped_faves);
            });
        })
    };

    let heart_button_class = if is_already_favorite {
        "rounded-full bg-bitcoin-orange text-white transition-colors p-4"
    } else {
        "rounded-full bg-white text-gray-500   transition-colors p-4"
    };

    let on_picture_click = {
        let image_index = image_index.clone();
        let listing = listing.clone();
        Callback::from(move |_| {
            let new_index = *image_index + 1;
            if new_index >= listing.images.len() {
                image_index.set(0);
            } else {
                image_index.set(new_index);
            }
        })
    };

    html! {
        <div class="relative aspect-square w-full bg-gray-200 h-fit">
            <img src={listing.images[*image_index].to_string()} class="object-cover h-full w-full" />
            <div class="absolute top-4 right-4 flex flex-row gap-6">
                <div onclick={on_picture_click}
                    class="rounded-full bg-white text-bitcoin-orange p-4">
                    <lucide_yew::Images class="w-6 h-6" />
                </div>
                <button onclick={on_favorite_click} class={heart_button_class}>
                    <lucide_yew::Heart class="w-6 h-6" />
                </button>
            </div>
            <div class="absolute bottom-4 right-4">
                <div class="rounded-full bg-white/70 text-bitcoin-orange p-2 border border-2 border-bitcoin-orange backdrop-blur-md">
                    <p>{listing.contract_type.to_string()}</p>
                </div>
            </div>

        </div>
    }
}
