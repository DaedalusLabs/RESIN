use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html! {
        <div class="flex flex-col gap-4">
            <img src="/public/wioso_logo.png" alt="logo" />
            <h2 class="text-2xl">{"Rent to own without a loan"}</h2>
            <button class="bg-blue-500 text-white p-2 rounded">{"Create an account"}</button>
            <button class="bg-blue-500 text-white p-2 rounded">{"Login with Nostr"}</button>
        </div>
    }
}

#[function_component(NostrLoginModal)]
pub fn nostr_login_modal() -> Html {
    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center">
            <div class="bg-white p-4 rounded">
                <h2>{"Login with Nostr"}</h2>
                <p>{"Choose an option below to access your account"}</p>
                <input type="text" placeholder="Username" />
                <input type="password" placeholder="Password" />
                <button class="bg-blue-500 text-white p-2 rounded">{"Login"}</button>
            </div>
        </div>
    }
}
