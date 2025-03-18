use std::rc::Rc;

use nostr_minions::relay_pool::NostrPoolStore;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListingData {
    has_loaded: bool,
    nostr_listings: Vec<nostro2::note::NostrNote>,
}

impl ListingData {
    pub fn finished_loading(&self) -> bool {
        self.has_loaded
    }
    pub fn listing_notes(&self) -> Vec<nostro2::note::NostrNote> {
        self.nostr_listings.clone()
    }
    pub fn listings(&self) -> Vec<resin_core::NostrListing> {
        self.nostr_listings
            .iter()
            .filter_map(|note| resin_core::NostrListing::try_from(note.clone()).ok())
            .collect()
    }
}
pub enum ListingDataAction {
    FinishedLoading,
    NewListing(nostro2::note::NostrNote),
}

impl Reducible for ListingData {
    type Action = ListingDataAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ListingDataAction::FinishedLoading => Rc::new(ListingData {
                has_loaded: true,
                ..(*self).clone()
            }),
            ListingDataAction::NewListing(listing) => Rc::new(ListingData {
                nostr_listings: {
                    let mut new_listings = (*self).nostr_listings.clone();
                    new_listings.push(listing);
                    new_listings
                },
                ..(*self).clone()
            }),
        }
    }
}

pub type ListingDataStore = UseReducerHandle<ListingData>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ListingDataChildren {
    pub children: Children,
}

#[function_component(ListingDataProvider)]
pub fn key_handler(props: &ListingDataChildren) -> Html {
    let ctx = use_reducer(|| ListingData {
        has_loaded: false,
        nostr_listings: vec![],
    });

    html! {
        <ContextProvider<ListingDataStore> context={ctx}>
            {props.children.clone()}
            <ListingDataSync />
        </ContextProvider<ListingDataStore>>
    }
}

#[function_component(ListingDataSync)]
pub fn commerce_data_sync() -> Html {
    let ctx = use_context::<ListingDataStore>().expect("Commerce context not found");
    let relay_ctx = use_context::<NostrPoolStore>().expect("Nostr context not found");
    let sub_id = use_state(|| String::new());

    let subscriber = relay_ctx.subscribe;
    let unique_notes = relay_ctx.unique_notes.clone();
    let relay_events = relay_ctx.relay_events.clone();

    let id_handle = sub_id.clone();
    use_effect_with((), move |()| {
        let listing_filter: nostro2::relay_events::NostrClientEvent =
            nostro2::subscriptions::NostrSubscription {
                kinds: vec![30403].into(),
                ..Default::default()
            }
            .into();
        if let nostro2::relay_events::NostrClientEvent::Subscribe(_, ref id, _) = listing_filter {
            id_handle.set(id.to_string());
        }
        subscriber.emit(listing_filter);
        || {}
    });

    let ctx_clone = ctx.clone();
    let id_handle = sub_id.clone();
    use_effect_with(relay_events, move |events| {
        if let Some(nostro2::relay_events::NostrRelayEvent::EndOfSubscription(.., id)) =
            events.last()
        {
            if *id == *id_handle {
                ctx_clone.dispatch(ListingDataAction::FinishedLoading);
            }
        }
        || {}
    });
    let ctx_clone = ctx.clone();

    use_effect_with(unique_notes, move |notes| {
        if let Some(note) = notes.last() {
            if note.kind == 30403 {
                if let Ok(_listing) = resin_core::NostrListing::try_from(note) {
                    ctx_clone.dispatch(ListingDataAction::NewListing(note.clone()));
                }
            }
        }
        || {}
    });
    html! {}
}
