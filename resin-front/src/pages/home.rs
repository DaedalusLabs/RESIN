use yew::prelude::*;

#[function_component(ResinHomePage)]
pub fn resin_home_page() -> Html {
    html! {
        <div class="h-full w-full flex flex-col p-8 gap-8">
            <img src="/public/wioso_logo.png" alt="placeholder" class="h-12" />
            <input type="text" placeholder="Search" class="p-2 border border-gray-300 rounded" />
            <div class="flex flex-col gap-4">
                <h3>{"Recent Searches"}</h3>
                <p>{"No recent searches"}</p>
            </div>
            <div class="flex flex-col gap-4">
                <h3>{"Recently Viewed"}</h3>
                <p>{"No recently viewed items"}</p>
            </div>
        </div>
    }
}

