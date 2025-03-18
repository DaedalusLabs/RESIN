use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsPageTemplateProps {
    pub children: Children,
    pub title: String,
}


#[function_component(SettingsPageTemplate)]
pub fn settings_page_template(props: &SettingsPageTemplateProps) -> Html {
    
    html! {
        <div class="h-full w-full flex flex-col p-8 gap-8">
            <yew_router::components::Link<crate::router::ResinRoute> to={crate::router::ResinRoute::Home}>
                <lucide_yew::ChevronLeft class="h-8 w-8" />            
            </yew_router::components::Link<crate::router::ResinRoute>>
            <h1>{props.title.clone()}</h1>
            {props.children.clone()}
        </div>
    }
}

