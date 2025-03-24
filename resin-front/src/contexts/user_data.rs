use std::rc::Rc;

use nostr_minions::{key_manager::NostrIdStore, relay_pool::NostrPoolStore};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResinUser {
    has_loaded: bool,
    user_profile: Option<nostro2::note::NostrNote>,
    user_favorites: Option<nostro2::note::NostrNote>,
}

impl ResinUser {
    #[must_use]
    pub const fn finished_loading(&self) -> bool {
        self.has_loaded
    }
    #[must_use]
    pub const fn profile_note(&self) -> Option<&nostro2::note::NostrNote> {
        self.user_profile.as_ref()
    }
    #[must_use]
    pub const fn favorites_note(&self) -> Option<&nostro2::note::NostrNote> {
        self.user_favorites.as_ref()
    }
    #[must_use]
    pub fn profile(&self) -> Option<resin_core::ResinUserProfile> {
        self.user_profile
            .as_ref()
            .and_then(|note| resin_core::ResinUserProfile::try_from(note).ok())
    }
    #[must_use]
    pub fn favorites(&self) -> Option<resin_core::ResinFavoriteListings> {
        self.user_favorites
            .as_ref()
            .and_then(|note| resin_core::ResinFavoriteListings::try_from(note).ok())
    }
}
pub enum ResinUserAction {
    FinishedLoading,
    LoadUserProfile(nostro2::note::NostrNote),
    LoadUserFavorites(nostro2::note::NostrNote),
}

impl Reducible for ResinUser {
    type Action = ResinUserAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ResinUserAction::FinishedLoading => Rc::new(Self {
                has_loaded: true,
                ..(*self).clone()
            }),
            ResinUserAction::LoadUserProfile(profile) => Rc::new(Self {
                user_profile: Some(profile),
                ..(*self).clone()
            }),
            ResinUserAction::LoadUserFavorites(favorites) => Rc::new(Self {
                user_favorites: Some(favorites),
                ..(*self).clone()
            }),
        }
    }
}

pub type ResinUserStore = UseReducerHandle<ResinUser>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ResinUserChildren {
    pub children: Children,
}

#[function_component(ResinUserProvider)]
pub fn key_handler(props: &ResinUserChildren) -> Html {
    let ctx = use_reducer(|| ResinUser {
        has_loaded: false,
        user_profile: None,
        user_favorites: None,
    });

    html! {
        <ContextProvider<ResinUserStore> context={ctx}>
            {props.children.clone()}
            <ResinUserSync />
        </ContextProvider<ResinUserStore>>
    }
}

#[function_component(ResinUserSync)]
pub fn commerce_data_sync() -> Html {
    let ctx = use_context::<ResinUserStore>().expect("Commerce context not found");
    let relay_ctx = use_context::<NostrPoolStore>().expect("Nostr context not found");
    let user_ctx = use_context::<NostrIdStore>().expect("User context not found");
    let sub_id = use_state(String::new);

    let subscriber = relay_ctx.subscribe;
    let unique_notes = relay_ctx.unique_notes;
    let relay_events = relay_ctx.relay_events;

    let id_handle = sub_id.clone();
    let ctx_clone = ctx.clone();
    use_effect_with(user_ctx.clone(), move |user_id| {
        if let Some(pubkey) = user_id.get_pubkey() {
            let listing_filter: nostro2::relay_events::NostrClientEvent =
                nostro2::subscriptions::NostrSubscription {
                    kinds: vec![10400, 10401].into(),
                    authors: vec![pubkey].into(),
                    ..Default::default()
                }
                .into();
            if let nostro2::relay_events::NostrClientEvent::Subscribe(_, ref id, _) = listing_filter
            {
                id_handle.set(id.to_string());
            }
            subscriber.emit(listing_filter);
        } else {
            ctx_clone.dispatch(ResinUserAction::FinishedLoading);
        }
        || {}
    });

    let ctx_clone = ctx.clone();
    use_effect_with(relay_events, move |events| {
        if let Some(nostro2::relay_events::NostrRelayEvent::EndOfSubscription(.., id)) =
            events.last()
        {
            if *id == *sub_id {
                ctx_clone.dispatch(ResinUserAction::FinishedLoading);
            }
        }
        || {}
    });

    use_effect_with(unique_notes, move |notes| {
        if let Some(giftwrapped) = notes.last().cloned() {
            if giftwrapped.kind == 10400 {
                let ctx_clone = ctx.clone();
                let user_id = user_ctx.clone();
                let giftwrapped = giftwrapped.clone();
                yew::platform::spawn_local(async move {
                    let gift = user_id
                        .unwrap_giftwrap(&giftwrapped)
                        .await
                        .expect("Failed to unwrap gift");
                    if let Ok(_listing) = resin_core::ResinUserProfile::try_from(&gift) {
                        ctx_clone.dispatch(ResinUserAction::LoadUserProfile(gift));
                    }
                });
            }
            if giftwrapped.kind == 10401 {
                yew::platform::spawn_local(async move {
                    let gift = user_ctx
                        .unwrap_giftwrap(&giftwrapped)
                        .await
                        .expect("Failed to unwrap gift");
                    if let Ok(_listing) = resin_core::ResinFavoriteListings::try_from(&gift) {
                        ctx.dispatch(ResinUserAction::LoadUserFavorites(gift));
                    }
                });
            }
        }
        || {}
    });
    html! {}
}
