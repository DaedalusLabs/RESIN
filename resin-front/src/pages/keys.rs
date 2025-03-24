use nostr_minions::key_manager::{NostrIdStore, NostrIdType};
use yew::prelude::*;

#[function_component(NostrKeysPage)]
pub fn nostr_keys_page() -> Html {
    let show_keys = use_state(|| false);
    html! {
        <div class="flex flex-col gap-4">
            <NostrPubkeySection />
            <RecoveryPhraseSection show_keys={show_keys.clone()} />
            <PrivateKeySection show_keys={show_keys.clone()} />
            <ShowKeysButton {show_keys} />
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShowKeysProps {
    pub show_keys: UseStateHandle<bool>,
}

#[function_component(ShowKeysButton)]
pub fn show_keys_button(props: &ShowKeysProps) -> Html {
    let ShowKeysProps { show_keys } = props;
    let onclick = {
        let show_keys = show_keys.clone();
        Callback::from(move |_| {
            show_keys.set(!*show_keys);
        })
    };

    let class = if **show_keys {
        "border border-gray-500 text-gray-500 bg-gray-100 p-2 rounded-lg flex items-center justify-center gap-2"
    } else {
        "border border-red-500 text-red-500 p-2 rounded-lg flex items-center justify-center gap-2"
    };
    let icon = if **show_keys {
        html! { <lucide_yew::Eye class="size-6" /> }
    } else {
        html! { <lucide_yew::EyeOff class="size-6" /> }
    };

    let text = if **show_keys {
        "Hide private keys"
    } else {
        "Show private keys"
    };

    html! {
        <button {onclick} {class}>
            {icon}
            <span>{text}</span>
        </button>
    }
}

#[function_component(NostrPubkeySection)]
pub fn nostr_pubkey_section() -> Html {
    let user_ctx = use_context::<NostrIdStore>()
        .expect("NostrPubkeySection must be a child of NostrIdProvider");
    let pubkey = user_ctx.get_pubkey().expect("NostrIdStore not found");
    let pubkey_modal = html! {
        <div class="flex flex-col gap-4 bg-white rounded-lg p-4">
            <h4 class="text-sm font-semibold text-[#444446]">{"Pubkey"}</h4>
            <p
                class="text-sm font-semibold"
                >{"This is your public key. It is used to identify you on the Nostr network."}</p>
        </div>
    };

    html! {
       <NostrKeysSection title="Your public key" modal={pubkey_modal}>
           <div class="relative w-full">
               <input
                   type="text"
                   class="pr-12 pl-4 w-full p-3 rounded-xl border border-1 bg-white border-[#D1D5DB] text-[#6B7280] focus:outline-none focus:ring-1 focus:ring-bitcoin-orange truncate"
                   value={pubkey.clone()} disabled={true} />

               <button
                   onclick={{
                       Callback::from(move |_| {
                           nostr_minions::browser_api::clipboard_copy(&pubkey);
                       })
                   }}
                   class="absolute right-3 top-1/2 -translate-y-1/2 z-30">
                   <lucide_yew::Copy class="size-6" />
               </button>
           </div>
       </NostrKeysSection>
    }
}

#[function_component(RecoveryPhraseSection)]
pub fn recovery_phrase_section(props: &ShowKeysProps) -> Html {
    let show_keys = *props.show_keys.clone();
    let user_ctx = use_context::<NostrIdStore>()
        .expect("RecoveryPhraseSection must be a child of NostrIdProvider");
    let nostr_key_state = use_state(|| None);
    let key_setter = nostr_key_state.setter();
    use_effect_with(user_ctx.clone(), move |nostr_id| {
        if nostr_id.get_identity().is_some() {
            let nostr_id = nostr_id.clone();
            yew::platform::spawn_local(async move {
                let mut nostr_key = nostr_id
                    .get_nostr_key()
                    .await
                    .expect("NostrIdStore not found");
                nostr_key.make_extractable();
                key_setter.set(Some(nostr_key));
            });
        }
        || {}
    });
    let recovery_phrase = html! {
        <div class="flex flex-col gap-4 bg-white p-4 rounded-lg">
            <h4
                class="text-sm font-semibold text-[#444446]"
                >{"Recovery Phrase"}</h4>
            <p
                class="text-sm font-semibold"
            >{"This is your recovery phrase. It is used to recover your identity on the Nostr network."}</p>
        </div>
    };

    let identity = user_ctx.get_identity().expect("NostrIdStore not found");
    if matches!(identity.signer, NostrIdType::Extension) {
        return html! {
            <p>{"Recovery phrase not available for extension identities"}</p>
        };
    }
    match nostr_key_state
        .as_ref()
        .map(|x| x.get_mnemonic(bip39::Language::English))
    {
        Some(Ok(mnemonic)) => {
            let words = mnemonic.split_whitespace().collect::<Vec<&str>>();
            html! {
                <NostrKeysSection title="Recovery Phrase" modal={recovery_phrase}>
                    <div class="grid grid-cols-2 grid-rows-6 gap-3">
                        {for words.iter().enumerate().take(12).map(|(i, word)| html! {
                            <div class="relative w-full">
                                <p
                                    class="absolute left-3 top-1/2 -translate-y-1/2 z-30 text-sm font-semibold"
                                    >{i + 1}{". "}</p>
                                <input
                                    type={if show_keys { "text" } else { "password" }}
                                    class="pr-12 pl-12 w-full p-3 rounded-xl border border-1 bg-white border-[#D1D5DB] text-[#6B7280] focus:outline-none focus:ring-1 focus:ring-bitcoin-orange truncate"
                                    value={(*word).to_string()} disabled={true} />

                                <button
                                    onclick={{
                                        let word = (*word).to_string();
                                        Callback::from(move |_| {
                                            nostr_minions::browser_api::clipboard_copy(&word);
                                        })
                                    }}
                                    class="absolute right-3 top-1/2 -translate-y-1/2 z-30">
                                    <lucide_yew::Copy class="size-6" />
                                </button>
                            </div>
                        })}
                    </div>
                </NostrKeysSection>
            }
        }
        _ => {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    }
}

#[function_component(PrivateKeySection)]
pub fn recovery_phrase_section(props: &ShowKeysProps) -> Html {
    let show_keys = *props.show_keys.clone();
    let user_ctx = use_context::<NostrIdStore>()
        .expect("RecoveryPhraseSection must be a child of NostrIdProvider");
    let nostr_key_state = use_state(|| None);
    let key_setter = nostr_key_state.setter();
    use_effect_with(user_ctx.clone(), move |nostr_id| {
        if nostr_id.get_identity().is_some() {
            let nostr_id = nostr_id.clone();
            yew::platform::spawn_local(async move {
                let mut nostr_key = nostr_id
                    .get_nostr_key()
                    .await
                    .expect("NostrIdStore not found");
                nostr_key.make_extractable();
                key_setter.set(Some(nostr_key));
            });
        }
        || {}
    });
    let private_key_modal = html! {
        <div class="flex flex-col gap-4 bg-white p-4 rounded-lg">
            <h4
                class="text-sm font-semibold text-[#444446]"
                >{"Private Key"}</h4>
            <p
                class="text-sm font-semibold"
                >{"This is your private key. It is used to sign transactions on the Nostr network."}</p>
        </div>
    };

    let identity = user_ctx.get_identity().expect("NostrIdStore not found");
    if matches!(identity.signer, NostrIdType::Extension) {
        return html! {
            <p>{"Private Key not available for extension identities"}</p>
        };
    }
    match nostr_key_state
        .as_ref()
        .map(nostro2_signer::keypair::NostrKeypair::get_nsec)
    {
        Some(Ok(nsec_key)) => {
            html! {
                <NostrKeysSection title="Private Key (nsec)" modal={private_key_modal}>
                    <div class="relative w-full">
                        <input
                            type={if show_keys { "text" } else { "password" }}
                            class="pr-12 pl-4 w-full p-3 rounded-xl border border-1 bg-white border-[#D1D5DB] text-[#6B7280] focus:outline-none focus:ring-1 focus:ring-bitcoin-orange truncate"
                            value={nsec_key.clone()} disabled={true} />

                        <button
                            onclick={{
                                Callback::from(move |_| {
                                    nostr_minions::browser_api::clipboard_copy(&nsec_key);
                                })
                            }}
                            class="absolute right-3 top-1/2 -translate-y-1/2 z-30">
                            <lucide_yew::Copy class="size-6" />
                        </button>
                    </div>
                </NostrKeysSection>
            }
        }
        _ => {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NostrKeysSectionProps {
    pub title: String,
    pub children: Children,
    pub modal: Html,
}
#[function_component(NostrKeysSection)]
pub fn nostr_keys_section(props: &NostrKeysSectionProps) -> Html {
    let NostrKeysSectionProps {
        title,
        children,
        modal,
    } = props;
    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <>
        <div class="flex flex-col gap-4">
            <div class="flex flex-row justify-between">
                <h4 class="text-sm font-semibold my-2 text-[#444446]"
                    >{title.clone()}</h4>
                <button {onclick}>
                    <lucide_yew::CircleHelp class="w-6 h-6 text--[#444446]" />
                </button>
            </div>
            {children.clone()}
        </div>
        <crate::components::modal::Modal {is_open}>
            {modal.clone()}
        </crate::components::modal::Modal>
        </>
    }
}
