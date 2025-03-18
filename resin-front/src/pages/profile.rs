use nostr_minions::{key_manager::NostrIdStore, widgets::forms::ImageUploadInput};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::ResinRoute;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let url_handle = use_state(|| None);
    let user_ctx =
        use_context::<NostrIdStore>().expect("ProfilePage must be a child of NostrIdProvider");

    let navigator = use_navigator().expect("ProfilePage must be a child of a Router");
    let Some(identity) = user_ctx.get_identity() else {
        navigator.push(&ResinRoute::Login);
        return html! { <></> };
    };
    html! {
        <div class="flex flex-col gap-4">
            <h4 >{"Picture"}</h4>
            <ImageUploadInput
                url_handle={url_handle.clone()}
                nostr_keys={identity.clone()}
                classes={classes!("w-32", "h-32")}
                input_id={"test-upload-input"}
            />
            <h4>{"Personal Information"}</h4>
            <input type="text"  />
            <input type="text"  />
            <input type="text"  />

            <input type="submit" value="Save"  />

        </div>
    }
}
