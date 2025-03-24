use nostr_minions::{
    key_manager::NostrIdStore, relay_pool::NostrPoolStore, widgets::forms::ImageUploadInput,
};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{contexts::user_data::ResinUserStore, router::ResinRoute};

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let user_ctx =
        use_context::<ResinUserStore>().expect("ProfilePage must be a child of NostrIdProvider");
    let profile = user_ctx.profile().unwrap_or_default();
    let url_handle = use_state(|| {
        if profile.image_url.is_empty() {
            None
        } else {
            Some(profile.image_url.clone())
        }
    });
    let relay_ctx =
        use_context::<NostrPoolStore>().expect("ProfilePage must be a child of NostrPoolProvider");
    let id_ctx =
        use_context::<NostrIdStore>().expect("ProfilePage must be a child of NostrIdProvider");
    let navigator = use_navigator().expect("ProfilePage must be a child of a Router");
    let Some(identity) = id_ctx.get_identity() else {
        navigator.push(&ResinRoute::Login);
        return html! { <></> };
    };

    let onsubmit = {
        let url_handle = url_handle.clone();
        let identity = identity.clone();
        let sender = relay_ctx.send_note;
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = nostr_minions::browser_api::HtmlForm::new(e).expect("Form not found");
            let first_name = form.input_value("first_name").unwrap_or_default();
            let last_name = form.input_value("last_name").unwrap_or_default();
            let email = form.input_value("email").unwrap_or_default();
            let image_url = (*url_handle).clone().unwrap_or_default();
            let user = resin_core::ResinUserProfile {
                first_name,
                last_name,
                email,
                image_url,
            };
            let mut nostr_note: nostro2_signer::nostro2::note::NostrNote = user.into();
            let identity = identity.clone();
            let sender = sender.clone();
            yew::platform::spawn_local(async move {
                let pubkey = identity.get_pubkey().await.expect("NostrIdStore not found");
                nostr_note.pubkey.clone_from(&pubkey);
                let mut giftwrap = identity
                    .create_giftwrap(nostr_note, 10400)
                    .await
                    .expect("Failed to sign note");
                identity
                    .sign_nip44(&mut giftwrap, pubkey)
                    .await
                    .expect("Failed to sign note");
                sender.emit(giftwrap);
            });
        })
    };
    let mut input_classes = classes!("size-32", "rounded-full", "bg-[(/public/OrangeUser.svg)]", "p-2");
    if profile.image_url.is_empty() {
        input_classes.push("opacity-100");
    } else {
        input_classes.push("hidden");
    }


    html! {
            <form {onsubmit}
                class="flex flex-col gap-4">
                <div class="flex flex-row gap-8 relative ">
                    <h4 class="text-sm font-semibold my-2">
                        {"Picture"}
                    </h4>
                    {url_handle.is_some().then(|| {
                        html! {
                            <button class="absolute bg-black text-white p-2 rounded-full scale-80 -bottom-12 left-24 z-50" type="button" onclick={{
                                let url_handle = url_handle.clone();
                                Callback::from(move |_| url_handle.set(None))
                            }}>
                                <lucide_yew::X class="size-5" />
                            </button>
                        }
                    })}
                </div>
                <ImageUploadInput
                    url_handle={url_handle.clone()}
                    nostr_keys={identity.clone()}
                    classes={input_classes}
                    image_classes={classes!("size-32", "rounded-full", "min-size-32")}
                    input_id={"test-upload-input"}
                    server_pubkey={"9079abf2786a4e0497873cf120290ec51cd57990ce04cc16a26f89f4f3f45128".to_string()}
                />
                <h4 class="text-sm font-semibold my-2">
                    {"Personal Information"}
                </h4>
                <input class="border border-1 bg-white border-[#D1D5DB] text-[#6B7280] rounded-lg py-3 px-5" type="text" name="first_name"  placeholder="First Name (optional)" value={profile.first_name} />
                <input class="border border-1 bg-white border-[#D1D5DB] text-[#6B7280] rounded-lg py-3 px-5" type="text" name="last_name"  placeholder="Last Name (optional)" value={profile.last_name} />
                <input class="border border-1 bg-white border-[#D1D5DB] text-[#6B7280] rounded-lg py-3 px-5" type="email" name="email"  placeholder="Email (optional)" value={profile.email} />

                <input class="bg-bitcoin-orange text-white py-2.5 px-5 w-fit rounded-lg mt-4" type="submit" value="Save"  />

            </form >
        }
}
