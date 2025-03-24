use yew::prelude::*;

use crate::components::search_bar::ListingsSearchBar;

#[function_component(ResinHomePage)]
pub fn resin_home_page() -> Html {
    html! {
        <div class="h-full w-full pt-16 space-y-8 overflow-hidden">
            <img src="/public/wioso_logo.png" alt="placeholder" class="w-64 md:w-64 px-7" />
            <div class="px-7">
                <ListingsSearchBar />
            </div>
            <div class="flex flex-col gap-4 px-7">
                <h2 class="text-2xl leading-tight font-extrabold">
                    {"Recent Searches"}
                </h2>
                <p>{"No recent searches"}</p>
            </div>
            <div class="flex flex-col gap-4 px-7">
                <h2 class="text-2xl leading-tight font-extrabold">
                    {"Recently Viewed"}
                </h2>
                <p>{"No recently viewed items"}</p>
            </div>
        </div>
    }
}

