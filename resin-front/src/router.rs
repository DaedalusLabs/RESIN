use yew::prelude::*;
use yew_router::prelude::*;

use crate::{home::ResinHomePage, profile::ProfilePage, search::NostrListingList, settings::SettingsPageTemplate};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum ResinRoute {
    #[at("/")]
    Home,
    #[at("/search")]
    Search,
    #[at("/favorites")]
    Favorites,
    #[at("/profile")]
    Profile,
    #[at("/keys")]
    Keys,
    #[at("/settings")]
    Settings,
    #[at("/help")]
    Help,
    #[at("/terms")]
    Terms,
    #[at("/my-resin")]
    MyResin,
    #[at("/listing/{id}")]
    Listing { id: String },
    #[at("/login")]
    Login,
}

#[function_component(ResinPages)]
pub fn resin_pages() -> Html {
    html! {
        <Switch<ResinRoute> render = { move |switch: ResinRoute| {
            match switch {
                ResinRoute::Home | ResinRoute::Search  | ResinRoute::Favorites | ResinRoute::MyResin => html! {
                    <div class="flex h-full w-full flex-col">
                        {match switch {
                            ResinRoute::Home => html! { < ResinHomePage /> },
                            ResinRoute::Search => html! { < NostrListingList /> },
                            ResinRoute::Favorites => html! { <div>{"Favorites"}</div> },
                            ResinRoute::MyResin => html! { <div>{"My Resin"}</div> },
                            _ => html! { <div>{"Home"}</div> },
                        }}
                        <Footer />
                    </div>
                },
                ResinRoute::Listing { id } => html! { <div>{format!("Listing: {}", id)}</div> },

                ResinRoute::Login => html! { <div>{"Login"}</div> },

                ResinRoute::Keys => 
                    html! { <SettingsPageTemplate title="Keys">{"Settigns"}</SettingsPageTemplate> },
                ResinRoute::Profile => 
                    html! { <SettingsPageTemplate title="Profile"><ProfilePage /></SettingsPageTemplate> },
                ResinRoute::Settings =>
                    html! { <SettingsPageTemplate title="Settings">{"Settigns"}</SettingsPageTemplate> },
                ResinRoute::Help =>
                    html! { <SettingsPageTemplate title="Help">{"Settigns"}</SettingsPageTemplate> },
                ResinRoute::Terms =>
                    html! { <SettingsPageTemplate title="Terms">{"Settigns"}</SettingsPageTemplate> },
            }}}
        />
    }
}
#[function_component(Footer)]
fn footer() -> Html {
    let is_open = use_state(|| false);

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <>
        <footer class="bg-gray-100 fixed z-30 bottom-0 w-full h-fit py-2">
            <div class="flex justify-center">
                <div class="flex flex-row space-x-4">
                    <yew_router::components::Link<ResinRoute> to={ResinRoute::Home}>
                        <img src="/public/house-icon.png" class="h-6 w-6" />
                    </yew_router::components::Link<ResinRoute>>
                    <yew_router::components::Link<ResinRoute> to={ResinRoute::Search}>
                        <img src="/public/house-icon.png" class="h-6 w-6" />
                    </yew_router::components::Link<ResinRoute>>
                    <yew_router::components::Link<ResinRoute> to={ResinRoute::MyResin}>
                        <img src="/public/daedalus_small_gray.png" class="h-6 w-6" />
                    </yew_router::components::Link<ResinRoute>>
                    <yew_router::components::Link<ResinRoute> to={ResinRoute::Favorites}>
                        <img src="/public/house-icon.png" class="h-6 w-6" />
                    </yew_router::components::Link<ResinRoute>>
                    <button {onclick}
                        class="h-6 w-6">
                        <img src="/public/house-icon.png" class="h-6 w-6" />
                    </button>
                </div>
            </div>
        </footer>
        <crate::components::drawer::LeftDrawer {is_open} >
            <SettingsDrawer />
        </crate::components::drawer::LeftDrawer>
        </>
    }
}

#[function_component(SettingsDrawer)]
fn settings_drawer() -> Html {
    let user_ctx = use_context::<nostr_minions::key_manager::NostrIdStore>()
        .expect("SettingsDrawer must be a child of NostrIdProvider");
    let logout_onclick = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_| {
            user_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::DeleteIdentity);
        })
    };
    html! {
        <div class="flex flex-col">
           <yew_router::components::Link<ResinRoute> to={ResinRoute::Profile}>
               <div>{"Profile"}</div>
           </yew_router::components::Link<ResinRoute>>
           <yew_router::components::Link<ResinRoute> to={ResinRoute::Keys}>
               <div>{"Keys"}</div>
           </yew_router::components::Link<ResinRoute>>
           <yew_router::components::Link<ResinRoute> to={ResinRoute::Settings}>
               <div>{"Settings"}</div>
           </yew_router::components::Link<ResinRoute>>
           <yew_router::components::Link<ResinRoute> to={ResinRoute::Help}>
               <div>{"Help"}</div>
           </yew_router::components::Link<ResinRoute>>
           <yew_router::components::Link<ResinRoute> to={ResinRoute::Terms}>
               <div>{"Terms"}</div>
           </yew_router::components::Link<ResinRoute>>
           {user_ctx.get_identity().is_some().then(|| {
              html! {
                <button onclick={logout_onclick} 
                    class="text-red-500">
                    {"Logout"}
                </button>
            }})}
        </div>
    }
}
