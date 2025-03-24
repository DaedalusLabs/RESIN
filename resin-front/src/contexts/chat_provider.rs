use std::rc::Rc;

use nostr_minions::{key_manager::NostrIdStore, relay_pool::NostrPoolStore};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResinChat {
    has_loaded: bool,
    pub messages: Vec<(bool, nostro2::note::NostrNote)>,
}

impl ResinChat {
    #[must_use]
    pub const fn finished_loading(&self) -> bool {
        self.has_loaded
    }

    /// Returns a copy of the messages in the chat 
    /// with a boolean indicating if the message is from the user or not 
    ///
    /// # Errors 
    ///
    /// Returns an error if the message is not a valid `NostrNote`
    #[allow(clippy::future_not_send)]
    pub async fn new_resin_dm(
        &self,
        content: String,
        subject: String,
        receiver_pubkey: String,
        identity: nostr_minions::key_manager::UserIdentity,
    ) -> Result<(nostro2::note::NostrNote, nostro2::note::NostrNote), wasm_bindgen::JsValue> {
        let id_pubkey = identity.get_pubkey().await.ok_or("No identity found")?;
        let mut unsigned_kind_14 = nostro2::note::NostrNote {
            kind: 14,
            content,
            pubkey: id_pubkey.clone(),
            ..Default::default()
        };
        unsigned_kind_14.tags.add_pubkey_tag(&receiver_pubkey);
        unsigned_kind_14
            .tags
            .add_custom_tag(nostro2::tags::NostrTag::Custom("subject"), &subject);
        let mut sealed_note = nostro2::note::NostrNote {
            kind: 13,
            content: unsigned_kind_14.to_string(),
            pubkey: id_pubkey.clone(),
            ..Default::default()
        };
        identity
            .sign_nip44(&mut sealed_note, receiver_pubkey.clone())
            .await?;
        let mut giftwrap = identity.create_giftwrap(sealed_note, 21059).await?;
        identity.sign_nip44(&mut giftwrap, receiver_pubkey).await?;
        Ok((giftwrap, unsigned_kind_14))
    }
}
pub enum ResinChatAction {
    FinishedLoading,
    NewResinMessage(nostro2::note::NostrNote),
    NewUserMessage(nostro2::note::NostrNote),
}

impl Reducible for ResinChat {
    type Action = ResinChatAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ResinChatAction::FinishedLoading => Rc::new(Self {
                has_loaded: true,
                ..(*self).clone()
            }),
            ResinChatAction::NewResinMessage(message) => Rc::new(Self {
                messages: self
                    .messages
                    .iter()
                    .map(|(is_resin, msg)| (*is_resin, msg.clone()))
                    .chain(std::iter::once((true, message)))
                    .collect(),
                ..(*self).clone()
            }),
            ResinChatAction::NewUserMessage(message) => Rc::new(Self {
                messages: self
                    .messages
                    .iter()
                    .map(|(is_resin, msg)| (*is_resin, msg.clone()))
                    .chain(std::iter::once((false, message)))
                    .collect(),
                ..(*self).clone()
            }),
        }
    }
}

pub type ResinChatStore = UseReducerHandle<ResinChat>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ResinChatChildren {
    pub children: Children,
}

#[function_component(ResinChatProvider)]
pub fn key_handler(props: &ResinChatChildren) -> Html {
    let ctx = use_reducer(|| ResinChat {
        has_loaded: false,
        messages: vec![],
    });

    html! {
        <ContextProvider<ResinChatStore> context={ctx}>
            {props.children.clone()}
            <ResinChatSync />
        </ContextProvider<ResinChatStore>>
    }
}

#[function_component(ResinChatSync)]
pub fn commerce_data_sync() -> Html {
    let ctx = use_context::<ResinChatStore>().expect("Commerce context not found");
    let relay_ctx = use_context::<NostrPoolStore>().expect("Nostr context not found");
    let user_ctx = use_context::<NostrIdStore>().expect("User context not found");
    let sub_id = use_state(String::new);

    let subscriber = relay_ctx.subscribe;
    let unique_notes = relay_ctx.unique_notes;
    let relay_events = relay_ctx.relay_events;

    let id_handle = sub_id.clone();
    use_effect_with(user_ctx.clone(), move |user_id| {
        if let Some(pubkey) = user_id.get_pubkey() {
            let mut chat_filter = nostro2::subscriptions::NostrSubscription {
                kinds: vec![21059].into(),
                authors: vec![
                    "9079abf2786a4e0497873cf120290ec51cd57990ce04cc16a26f89f4f3f45128".to_string(),
                ]
                .into(),
                ..Default::default()
            };
            chat_filter.add_tag("#p", pubkey.as_str());
            let event: nostro2::relay_events::NostrClientEvent = chat_filter.into();
            if let nostro2::relay_events::NostrClientEvent::Subscribe(_, ref id, _) = event {
                id_handle.set(id.to_string());
            }
            subscriber.emit(event);
        }
        || {}
    });

    let ctx_clone = ctx.clone();
    use_effect_with(relay_events, move |events| {
        if let Some(nostro2::relay_events::NostrRelayEvent::EndOfSubscription(.., id)) =
            events.last()
        {
            if *id == *sub_id {
                ctx_clone.dispatch(ResinChatAction::FinishedLoading);
            }
        }
        || {}
    });

    use_effect_with(unique_notes, move |notes| {
        if let Some(giftwrapped) = notes.last().cloned() {
            yew::platform::spawn_local(async move {
                if giftwrapped.kind != 21059 {
                    return;
                }
                let user_pubkey = user_ctx.get_pubkey().unwrap_or_default();
                let Some(addressed_to) = giftwrapped.tags.find_first_tagged_pubkey() else {
                    return;
                };
                if addressed_to == user_pubkey {
                    let Ok(gift) = user_ctx.unwrap_giftwrap(&giftwrapped).await else {
                        return;
                    };
                    let Ok(decrypted) = user_ctx.decrypt_note(&gift).await else {
                        return;
                    };
                    let Ok(note_msg) = decrypted.parse::<nostro2::note::NostrNote>() else {
                        return;
                    };
                    ctx.dispatch(ResinChatAction::NewResinMessage(note_msg));
                }
            });
        }
        || {}
    });
    html! {}
}
