use crate::components::{drawer::BottomDrawer, modal::Modal};
use nostr_minions::{browser_api::IdbStoreManager, key_manager::NostrIdStore};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let user_ctx = use_context::<NostrIdStore>().expect("NostrIdStore not found");
    let navigator = use_navigator().expect("Navigator not found");

    use_effect_with(user_ctx, move |id| {
        if id.get_identity().is_some() {
            navigator.push(&crate::router::ResinRoute::Home);
        }
        || {}
    });

    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(true))
    };

    let new_account_is_open = use_state(|| false);
    let new_account_onclick = {
        let new_account_is_open = new_account_is_open.clone();
        Callback::from(move |_| new_account_is_open.set(true))
    };
    html! {
        <div class="relative h-screen w-full bg-[#3C3C3D]">
            <img class="absolute h-screen w-full object-cover z-10 opacity-10" src="/public/BG_Hexagon_Dark.svg" alt="background" />
            <div class="size-full flex flex-col gap-4 absolute z-20 justify-between items-center py-16 px-7">
                <img src="/public/wioso_logo.png" alt="logo" class="mt-4 h-18" />
                <div class="relative flex flex-col justify-center items-center gap-8">
                    <h2 class="leading-tight text-4xl md:text-6xl font-bold text-center text-white px-4 max-w-sm md:max-w-md">
                        {"Rent to own without a loan"}
                    </h2>
                    <button onclick={new_account_onclick}
                        class="bg-bitcoin-orange text-white py-3 px-5 rounded-xl font-medium">
                        {"Create an account"}
                    </button>
                    <img src="/public/Vector.png" alt="arrow" class="absolute bottom-3.5 right-14 md:right-4 rotate-6.6" />
                    <img src="/public/Arrow.png" alt="arrow" class="absolute right-6 md:-right-4 bottom-3 rotate-6.6" />
                </div>
                <button {onclick}
                    class="border border-1 border-gray-200 text-white py-3 px-5 rounded-xl font-medium">
                    {"Login with Nostr"}
                </button>
            </div>
            <Modal {is_open}>
                <NostrLoginModal />
            </Modal >
            <Modal is_open={new_account_is_open}>
                <NewAccountModal />
            </Modal >
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NostrLoginModalProps {
    pub is_open: UseStateHandle<bool>,
}

#[function_component(NewAccountModal)]
pub fn new_account_modal() -> Html {
    let user_ctx = use_context::<NostrIdStore>().expect("NostrIdStore not found");
    let onclick = {
        Callback::from(move |_| {
            let user_ctx = user_ctx.clone();
            let new_keys = nostro2_signer::keypair::NostrKeypair::generate(true);
            let pubkey = new_keys.public_key();
            yew::platform::spawn_local(async move {
                match nostr_minions::key_manager::UserIdentity::from_new_keys(new_keys).await {
                    Ok(id) => {
                        id.clone()
                            .save_to_store()
                            .await
                            .expect("Error saving identity");
                        user_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                            pubkey, id,
                        ));
                    }
                    Err(e) => {
                        gloo::console::error!("Error loading identity: ", e);
                    }
                }
            });
        })
    };
    html! {
        <div class="bg-white p-4 rounded-2xl flex flex-col gap-4 shadow-lg flex flex-col justify-center items-center text-center">
            <img src="/public/NostrPurple.png" alt="logo" class="w-16 h-16" />
            <h2 class="text-sm font-black text-gray-900">{"Creating NOSTR account "}</h2>
            <div class="px-12">
                <p class="text-sm text-gray-500">
                    {"A new key will be created and saved locally on your device."}
                </p>
                <p class="text-sm text-gray-500">
                    {"Find options to backup this key in your app settings"}
                </p>
            </div>
            <button {onclick}
                class="bg-bitcoin-orange text-white py-2.5 px-5 rounded-lg">{"I Understand"}
            </button>
        </div>
    }
}

#[function_component(NostrLoginModal)]
pub fn nostr_login_modal() -> Html {
    html! {
        <div class="bg-white p-4 rounded-xl flex flex-col gap-4 ">
            <h2 class="text-xl font-semibold text-gray-900">
                {"Login with Nostr"}
            </h2>
            <p class="text-sm text-[#303031]">
                {"Choose an option below to access your account"}
            </p>
            <div class="flex flex-col gap-2 text-center">
                <BrowserExtensionForm />
                <AuthDrawer title="Use Private Key (NSEC)">
                    <NostrPrivateKeyForm />
                </AuthDrawer>
                <AuthDrawer title="Use Recovery Phrase (12 words)">
                    <NostrRecoveryPhraseForm />
                </AuthDrawer>
            </div>
            <div class="flex flex-col text-center">
                <p class="text-xs text-gray-500">{"Or "}<bold class="text-bitcoin-orange">{"create an account"}</bold>{" if you don't have one yet."}</p>
                <p class="text-xs text-gray-500">{"Have questions? "}<bold class="text-bitcoin-orange">{"Contact us"}</bold>{" for assistance"}</p>
            </div>
        </div>
    }
}

#[function_component(BrowserExtensionForm)]
pub fn browser_extension_form() -> Html {
    let user_ctx = use_context::<NostrIdStore>().expect("NostrIdStore not found");
    let is_extension_available = use_state(|| false);

    let extension_available = is_extension_available.clone();
    use_effect_with(user_ctx.clone(), move |_| {
        yew::platform::spawn_local(async move {
            if nostr_minions::key_manager::NostrSignerExtension::new()
                .await
                .is_ok()
            {
                extension_available.set(true);
            }
        });
        || {}
    });

    let extension_button_class = if *is_extension_available {
        "border border-bitcoin-orange text-bitcoin-orange text-sm font-medium py-2.5 px-5 rounded-xl"
    } else {
        "border border-gray-500 text-gray-500 text-sm font-medium py-2.5 px-5 rounded-xl cursor-not-allowed"
    };

    let extension_button_onclick = {
        Callback::from(move |_| {
            let user_ctx = user_ctx.clone();
            yew::platform::spawn_local(async move {
                match nostr_minions::key_manager::UserIdentity::new_extension_identity().await {
                    Ok(id) => {
                        id.clone()
                            .save_to_store()
                            .await
                            .expect("Error saving identity");
                        user_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                            id.get_pubkey().await.unwrap(),
                            id,
                        ));
                    }
                    Err(e) => {
                        gloo::console::error!("Error loading identity: ", e);
                    }
                }
            });
        })
    };
    html! {
        <button onclick={extension_button_onclick}
            class={extension_button_class}>{"Use Browser Extension"}</button>
    }
}

#[function_component(NostrPrivateKeyForm)]
pub fn nostr_private_key_form() -> Html {
    let user_ctx = use_context::<NostrIdStore>().expect("NostrIdStore not found");

    let onsubmit = {
        let user_ctx = user_ctx;
        Callback::from(move |event: yew::events::SubmitEvent| {
            event.prevent_default();
            let form =
                nostr_minions::browser_api::HtmlForm::new(event).expect("Error parsing form");
            let value = form.input_value("password").expect("Error parsing input");
            let Ok(mut keys) = nostro2_signer::keypair::NostrKeypair::try_from(value) else {
                gloo::console::error!("Error parsing keypair");
                return;
            };
            keys.make_extractable();

            let user_ctx = user_ctx.clone();
            yew::platform::spawn_local(async move {
                let pubkey = keys.public_key();
                match nostr_minions::key_manager::UserIdentity::from_new_keys(keys).await {
                    Ok(id) => {
                        id.clone()
                            .save_to_store()
                            .await
                            .expect("Error saving identity");
                        user_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                            pubkey, id,
                        ));
                    }
                    Err(e) => {
                        gloo::console::error!("Error loading identity: ", e);
                    }
                }
            });
        })
    };
    html! {
        <form {onsubmit}
            class="flex flex-col gap-4 p-8">
            <h3 class="text-xl font-bold text-[#303031]">{"Log in with your NOSTR private key"}</h3>
            <input
                class="px-4 w-full p-3 rounded-xl border border-1 bg-gray-100 border-[#D1D5DB] text-[#6B7280] focus:outline-none focus:ring-1 focus:ring-bitcoin-orange truncate"
                name="password" type="password" placeholder="Enter key in hex or nsec format" />
            <button class="bg-bitcoin-orange text-white py-2.5 px-5 rounded-lg flex flex-row items-center justify-center">
            <img src="/public/SignIn.svg" alt="logo" class="size-5 mr-2" />
            <p class="text-sm font-medium">
                {"Log In"}
            </p>
            </button>
        </form>
    }
}

#[function_component(NostrRecoveryPhraseForm)]
pub fn nostr_recovery_phrase_form() -> Html {
    let onsubmit = {
        let user_ctx = use_context::<NostrIdStore>().expect("NostrIdStore not found");
        Callback::from(move |event: yew::events::SubmitEvent| {
            event.prevent_default();
            let form =
                nostr_minions::browser_api::HtmlForm::new(event).expect("Error parsing form");
            let mut words = Vec::new();
            for i in 0..12 {
                let word = form
                    .input_value(&i.to_string())
                    .expect("Error parsing input");
                words.push(word);
            }
            let Ok(keys) = nostro2_signer::keypair::NostrKeypair::parse_mnemonic(
                words
                    .iter()
                    .fold(String::new(), |acc, x| acc + x + " ")
                    .as_str(),
                bip39::Language::English,
                true,
            ) else {
                gloo::console::error!("Error parsing mnemonic");
                return;
            };

            let user_ctx = user_ctx.clone();
            yew::platform::spawn_local(async move {
                match nostr_minions::key_manager::UserIdentity::from_new_keys(keys).await {
                    Ok(id) => {
                        id.clone()
                            .save_to_store()
                            .await
                            .expect("Error saving identity");
                        user_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                            id.get_pubkey().await.unwrap(),
                            id,
                        ));
                    }
                    Err(e) => {
                        gloo::console::error!("Error loading identity: ", e);
                    }
                }
            });
        })
    };
    html! {
        <form {onsubmit}
            class="flex flex-col gap-4 p-8">
            <h3 class="text-xl font-bold text-[#303031]">{"Log in with your recovery phrase"}</h3>
            <div class="grid grid-cols-2 grid-rows-6 gap-1">
                {for (0..12).map(|i| html! {
                    <input
                        class="px-4 w-full p-3 rounded-xl border border-1 bg-gray-100 border-[#D1D5DB] text-[#6B7280] focus:outline-none focus:ring-1 focus:ring-bitcoin-orange truncate"
                        type="text" name={i.to_string()} placeholder={i.to_string()} />
                })}
            </div>
            <button class="bg-bitcoin-orange text-white py-2.5 px-5 rounded-lg flex flex-row items-center justify-center">
                <img src="/public/SignIn.svg" alt="logo" class="size-5 mr-2" />
                <p class="text-sm font-medium">
                    {"Log In"}
                </p>
            </button>
        </form>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AuthDrawerProps {
    pub title: String,
    pub children: Children,
}

#[function_component(AuthDrawer)]
pub fn auth_drawer(props: &AuthDrawerProps) -> Html {
    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(true))
    };
    html! {
        <>
        <button {onclick}
        class="border border-bitcoin-orange text-bitcoin-orange text-sm font-medium py-2.5 px-5 rounded-xl"
        >{&props.title}</button>
        <BottomDrawer {is_open}>
            { for props.children.iter() }
        </BottomDrawer>
        </>
    }
}
