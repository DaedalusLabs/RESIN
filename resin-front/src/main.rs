use resin_front::ListingDataProvider;
use yew::prelude::*;
use yew_router::BrowserRouter;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppNostrProviders>
                <ListingDataProvider>
                    <div class="h-screen w-screen">
                        <resin_front::ResinPages />
                    </div>
                </ListingDataProvider>
            </AppNostrProviders>
        </BrowserRouter>
    }
}

#[function_component(AppNostrProviders)]
fn app_nostr_providers(props: &yew::html::ChildrenProps) -> Html {
    let relays = vec![
        nostr_minions::relay_pool::UserRelay {
            url: "wss://relay.illuminodes.com".to_string(),
            read: true,
            write: true,
        },
        nostr_minions::relay_pool::UserRelay {
            url: "wss://relay.arrakis.lat".to_string(),
            read: true,
            write: false,
        },
    ];
    html! {
        <nostr_minions::relay_pool::RelayProvider {relays}>
            <nostr_minions::key_manager::NostrIdProvider>
                {props.children.clone()}
            </nostr_minions::key_manager::NostrIdProvider>
        </nostr_minions::relay_pool::RelayProvider>
    }
}
