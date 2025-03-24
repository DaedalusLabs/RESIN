use crate::contexts::{
    chat_provider::ResinChatStore, provider::ListingDataStore, user_data::ResinUserStore,
};
use std::rc::Rc;

use nostr_minions::{key_manager::NostrIdStore, relay_pool::NostrPoolStore};
use yew::prelude::*;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadingCtx {
    relays_loaded: bool,
    user_loaded: bool,
    key_loaded: bool,
    listings_loaded: bool,
    chat_loaded: bool,
    animation_done: bool,
}

impl LoadingCtx {
    #[must_use]
    pub const fn finished_loading(&self) -> bool {
        self.relays_loaded
            && self.key_loaded
            && self.listings_loaded
            && self.animation_done
            //&& self.user_loaded
            //&& self.chat_loaded
    }
}
pub enum LoadingCtxAction {
    LoadedRelays,
    LoadedUser,
    LoadedKey,
    LoadedListings,
    LoadedChat,
    LoadedAnimation,
}

impl Reducible for LoadingCtx {
    type Action = LoadingCtxAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LoadingCtxAction::LoadedRelays => Rc::new(Self {
                relays_loaded: true,
                ..(*self).clone()
            }),
            LoadingCtxAction::LoadedUser => Rc::new(Self {
                user_loaded: true,
                ..(*self).clone()
            }),
            LoadingCtxAction::LoadedKey => Rc::new(Self {
                key_loaded: true,
                ..(*self).clone()
            }),
            LoadingCtxAction::LoadedListings => Rc::new(Self {
                listings_loaded: true,
                ..(*self).clone()
            }),
            LoadingCtxAction::LoadedAnimation => Rc::new(Self {
                animation_done: true,
                ..(*self).clone()
            }),
            LoadingCtxAction::LoadedChat => Rc::new(Self {
                chat_loaded: true,
                ..(*self).clone()
            }),
        }
    }
}

pub type LoadingCtxStore = UseReducerHandle<LoadingCtx>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LoadingCtxChildren {
    pub children: Children,
}

#[function_component(LoadingCtxProvider)]
pub fn key_handler(props: &LoadingCtxChildren) -> Html {
    let finished_loading = use_state(|| false);
    let ctx = use_reducer(|| LoadingCtx {
        relays_loaded: false,
        user_loaded: false,
        key_loaded: false,
        listings_loaded: false,
        chat_loaded: false,
        animation_done: false,
    });
    let id_ctx = use_context::<NostrIdStore>().expect("User context not found");
    let ctx_clone = ctx.clone();
    use_effect_with(id_ctx, move |id_ctx| {
        if id_ctx.loaded() {
            ctx_clone.dispatch(LoadingCtxAction::LoadedKey);
        }
        || {}
    });
    let relay_ctx = use_context::<NostrPoolStore>().expect("Nostr context not found");
    let ctx_clone = ctx.clone();
    use_effect_with(relay_ctx, move |relay_ctx| {
        if !relay_ctx.relay_events.is_empty() {
            ctx_clone.dispatch(LoadingCtxAction::LoadedRelays);
        }
        || {}
    });

    let user_ctx = use_context::<ResinUserStore>().expect("User context not found");
    let ctx_clone = ctx.clone();
    use_effect_with(user_ctx, move |user_ctx| {
        if user_ctx.finished_loading() {
            ctx_clone.dispatch(LoadingCtxAction::LoadedUser);
        }
        || {}
    });

    let listing_ctx = use_context::<ListingDataStore>().expect("Listing context not found");
    let ctx_clone = ctx.clone();
    use_effect_with(listing_ctx, move |listing_ctx| {
        if listing_ctx.finished_loading() {
            ctx_clone.dispatch(LoadingCtxAction::LoadedListings);
        }
        || {}
    });

    let chat_ctx = use_context::<ResinChatStore>().expect("Chat context not found");
    let ctx_clone = ctx.clone();
    use_effect_with(chat_ctx, move |chat_ctx| {
        if chat_ctx.finished_loading() {
            ctx_clone.dispatch(LoadingCtxAction::LoadedChat);
        }
        || {}
    });

    let finished_loading_clone = finished_loading.clone();
    use_effect_with(ctx.clone(), move |context| {
        if context.relays_loaded
            && context.key_loaded
            && context.listings_loaded
            // && context.user_loaded
            // && context.chat_loaded
        {
            let context = context.clone();
            yew::platform::spawn_local(async move {
                finished_loading_clone.set(true);
                gloo::timers::future::sleep(std::time::Duration::from_millis(280)).await;
                context.dispatch(LoadingCtxAction::LoadedAnimation);
            });
        }
        || {}
    });
    let class = if *finished_loading {
        "h-screen w-screen bg-bitcoin-orange flex justify-center items-center transition duration-300 transition-all  -translate-x-full"
    } else {
        "h-screen w-screen bg-bitcoin-orange flex justify-center items-center transition duration-300 transition-all  translate-x-0"
    };

    if !ctx.finished_loading() {
        return html! {
        <div id="inner-loading-screen" {class}>
            <img src="/public/daedalus_large_white.png" class="h-32" />
        </div>
        };
    }

    html! {
        <ContextProvider<LoadingCtxStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<LoadingCtxStore>>
    }
}
