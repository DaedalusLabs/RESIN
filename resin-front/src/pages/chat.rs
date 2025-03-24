use nostr_minions::{key_manager::NostrIdStore, relay_pool::NostrPoolStore};
use wasm_bindgen::JsCast;
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

use crate::{
    contexts::{
        chat_provider::{ResinChatAction, ResinChatStore},
        user_data::ResinUserStore,
    },
    router::ResinRoute,
};

#[function_component(ChatPage)]
pub fn chat_page() -> Html {
    let chat_ctx = use_context::<ResinChatStore>().expect("Chat context not found");
    let user_ctx = use_context::<ResinUserStore>().expect("User context not found");
    let id_ctx = use_context::<NostrIdStore>().expect("Id context not found");
    let relay_ctx = use_context::<NostrPoolStore>().expect("Relay context not found");
    let navigator = use_navigator().expect("ChatPage must be a child of a Router");
    let messages_end_ref = use_node_ref();
    let new_message = use_state(String::new);

    let end_ref_handle = messages_end_ref.clone();
    use_effect_with(chat_ctx.messages.clone(), move |_| {
        if let Some(end_ref) = end_ref_handle.get() {
            end_ref
                .unchecked_into::<web_sys::Element>()
                .scroll_into_view();
        }
        || {}
    });

    let send_message = {
        let chat_ctx = chat_ctx.clone();
        let new_message = new_message.clone();
        let user_ctx = user_ctx.clone();
        let relay_sender = relay_ctx.send_note;
        Callback::from(move |_e: MouseEvent| {
            let Some(user_id) = id_ctx.get_identity().cloned() else {
                navigator.push(&ResinRoute::Login);
                return;
            };

            let Some(_) = user_ctx.profile().map(|profile| profile.image_url) else {
                navigator.push(&ResinRoute::Profile);
                return;
            };
            if new_message.trim().is_empty() {
                return;
            }
            let new_message = new_message.clone();
            let chat_ctx = chat_ctx.clone();
            let relay_sender = relay_sender.clone();
            spawn_local(async move {
                let Ok(new_dm) = chat_ctx
                    .new_resin_dm(
                        (*new_message).clone(),
                        "Support Request".to_string(),
                        "9079abf2786a4e0497873cf120290ec51cd57990ce04cc16a26f89f4f3f45128"
                            .to_string(),
                        user_id,
                    )
                    .await
                else {
                    gloo::console::error!("Failed to create new message");
                    return;
                };
                relay_sender.emit(new_dm.0);
                chat_ctx.dispatch(ResinChatAction::NewUserMessage(new_dm.1));
            });
        })
    };
    let profile = user_ctx.profile().unwrap_or_default();

    html! {
        <div class="h-full w-full flex flex-col pt-16 gap-8 overflow-hidden">
            <h2 class="text-2xl leading-tight font-extrabold px-7">
                {"Messages"}
            </h2>

            <div class="flex-1 overflow-y-auto px-7 space-y-4 w-full">
                {for chat_ctx.messages.iter().cloned().map(|(is_system, message)| {
                    let image_url = if is_system {
                      "/public/BrandIcon_RESIN.png".to_string()
                    } else {
                        profile.image_url.clone()
                    };
                    html! {
                        <ChatBubble {is_system} {message} {image_url} />
                    }
                })}
                <div ref={messages_end_ref} />
            </div>

            <div class="bg-white p-4">
                <div class="relative">
                    <textarea
                        value={(*new_message).clone()}
                        oninput={{
                            let message_setter = new_message.setter();
                            Callback::from(move |e: InputEvent| {
                                message_setter.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                            })
                        }}
                        placeholder="Type your message..."
                        class="pr-12 pl-4 w-full p-3 rounded-xl border-1 border-gray-200 rounded-lg focus:outline-none focus:ring-1 focus:ring-bitcoin-orange "
                        onkeydown={{
                              let send_message = send_message.clone();
                              Callback::from(move |e: KeyboardEvent| {
                              if e.key() == "Enter" && !e.shift_key() {
                                  e.prevent_default();
                                  send_message.emit(e.unchecked_into());
                              }
                        })}}
                    />
                    <button
                        onclick={send_message}  class="absolute right-3 top-1/2 -translate-y-1/2"
                        disabled={new_message.trim().is_empty()}>
                        <lucide_yew::Send class="size-6" />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct ChatBubbleProps {
    message: nostro2::note::NostrNote,
    is_system: bool,
    image_url: String,
}

#[function_component(ChatBubble)]
pub fn chat_bubble(props: &ChatBubbleProps) -> Html {
    let ChatBubbleProps {
        message,
        is_system,
        image_url,
    } = props;
    #[allow(clippy::cast_precision_loss)]
    let date = web_sys::js_sys::Date::new(&web_sys::wasm_bindgen::JsValue::from_f64(
        message.created_at as f64,
    ));

    let root_class = if *is_system {
        "flex gap-2"
    } else {
        "flex gap-2 justify-end"
    };
    let icon_class = if *is_system {
        "size-6 rounded-full"
    } else {
        "size-6 rounded-full order-last"
    };
    let holder_class = if *is_system {
        "flex flex-col items-start"
    } else {
        "flex flex-col items-end"
    };
    let bubble_class = if *is_system {
        "rounded-lg px-4 py-2 bg-bitcoin-orange text-white rounded-tl-none max-w-xs break-words"
    } else {
        "rounded-lg px-4 py-2 bg-black text-white rounded-tr-none w-fit max-w-xs break-words"
    };

    html! {
        <div class={root_class}>
            <img src={image_url.to_string()} class={icon_class} />
            <div key={message.id.clone().unwrap_or_default()} class={holder_class} >
                <span class="text-xs text-gray-500 mt-1">
                    {date.to_locale_time_string("en-US").as_string().unwrap_or_default()}
                </span>
                <div class={bubble_class}>
                  {&message.content}
                </div>
            </div>
        </div>
    }
}
