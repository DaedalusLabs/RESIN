use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};
use yew::html::Children;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub title: String,
    pub children: Children,
    pub is_open: bool,
    pub on_close: Callback<()>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let is_closing = use_state(|| false);
    let modal_ref = use_node_ref();

    let on_close = props.on_close.clone();
    let is_open = props.is_open;

    // Handle closing animation
    {
        let is_closing = is_closing.clone();
        use_effect_with(is_open.clone(), move |is_open| {
            if !*is_open && !*is_closing {
                on_close.emit(());
            }
            || {}
        });
    }

    // Handle keydown and mousedown events
    {
        let modal_ref = modal_ref.clone();
        let is_closing = is_closing.clone();
        let on_close = props.on_close.clone();
        use_effect_with(is_open.clone(), move |is_open| {
            if *is_open {
                let document = web_sys::window().unwrap().document().unwrap();

                let on_close_clone = on_close.clone();
                let is_closing_clone = is_closing.clone();
                // Keydown event listener
                let keydown_callback = Callback::from(move |e: KeyboardEvent| {
                    if e.key() == "Escape" {
                        is_closing_clone.set(true);
                        let is_closing_clone = is_closing_clone.clone();
                        let on_close = on_close_clone.clone();
                        let timeout = Timeout::new(300, move || {
                            is_closing_clone.set(false);
                            on_close.emit(());
                        });
                        timeout.forget();
                    }
                });
                let _listener_keydown = EventListener::new(&document, "keydown", move |e| {
                    keydown_callback.emit(e.dyn_ref::<KeyboardEvent>().unwrap().clone());
                });

                // Mousedown event listener
                let mousedown_callback = Callback::from(move |e: MouseEvent| {
                    if let Some(modal_element) = modal_ref.cast::<web_sys::HtmlElement>() {
                        if let Some(target) = e.target() {
                            if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                                if !modal_element.contains(Some(&target_element)) {
                                    is_closing.set(true);
                                    let on_close = on_close.clone();
                                    let is_closing = is_closing.clone();
                                    let timeout = Timeout::new(300, move || {
                                        is_closing.set(false);
                                        on_close.emit(());
                                    });
                                    timeout.forget();
                                }
                            }
                        }
                    }
                });
                let _listener_mousedown = EventListener::new(&document, "mousedown", move |e| {
                    mousedown_callback.emit(e.dyn_ref::<MouseEvent>().unwrap().clone());
                });
            }
            || {}
        });
    }

    if !is_open && !*is_closing {
        return html! {};
    }

    let backdrop_class = if *is_closing {
        "fixed inset-0 z-50 flex items-center justify-center p-4 transition-all duration-300 ease-in-out bg-black/0 opacity-0"
    } else {
        "fixed inset-0 z-50 flex items-center justify-center p-4 transition-all duration-300 ease-in-out bg-black/50 opacity-100"
    };

    let modal_class = if *is_closing {
        "relative w-full max-w-md rounded-lg bg-white p-6 shadow-lg dark:bg-gray-800 transition-all duration-300 ease-in-out translate-y-4 scale-95 opacity-0"
    } else {
        "relative w-full max-w-md rounded-lg bg-white p-6 shadow-lg dark:bg-gray-800 transition-all duration-300 ease-in-out translate-y-0 scale-100 opacity-100"
    };

    let on_close = props.on_close.clone();
    let onclick = Callback::from(move |_| {
        is_closing.set(true);
        let on_close = on_close.clone();
        let is_closing = is_closing.clone();
        let timeout = Timeout::new(300, move || {
            is_closing.set(false);
            on_close.emit(());
        });
        timeout.forget();
    });

    html! {
        <div class={backdrop_class}>
            <div
                ref={modal_ref}
                class={modal_class}
                role="dialog"
                aria-modal="true"
                aria-labelledby="modal-title"
            >
                <div class="flex items-center justify-between">
                    <h2 id="modal-title" class="text-xl font-semibold">
                        {&props.title}
                    </h2>
                    <button
                        class="h-8 w-8 rounded-full flex items-center justify-center hover:bg-gray-100 dark:hover:bg-gray-700"
                        onclick={onclick.clone()}
                        aria-label="Close"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    </button>
                </div>
                <div class="mt-4">
                    {for props.children.iter()}
                </div>
            </div>
        </div>
    }
}
