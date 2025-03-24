use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsPageTemplateProps {
    pub children: Children,
    pub title: String,
}


#[function_component(SettingsPageTemplate)]
pub fn settings_page_template(props: &SettingsPageTemplateProps) -> Html {
    
    html! {
        <div class="relative bg-white h-screen w-full">
            <img class="absolute h-screen w-full object-cover z-10 opacity-5" src="/public/BG_Hexagon_White.svg" alt="background" />
            <div class="absolute h-full w-full flex flex-col pt-16 gap-8 overflow-hidden px-7 z-20">
                <yew_router::components::Link<crate::router::ResinRoute> to={crate::router::ResinRoute::Home}>
                    <lucide_yew::ChevronLeft class="size-8" />            
                </yew_router::components::Link<crate::router::ResinRoute>>
                <h2 class="text-2xl leading-tight font-extrabold">
                    {props.title.clone()}
                </h2>
                {props.children.clone()}
            </div>
        </div>
    }
}

