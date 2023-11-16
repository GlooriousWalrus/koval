use leptos::{ev::MouseEvent, *};

use crate::components::variants::base::ClassVariant;
use crate::OptionMaybeSignal;

#[component]
pub fn Button<F>(
    on_click: F,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            type="button"
            id=id
            class=move || format!("{} {}", variant.get(), class.get())
            style=style
            aria-disabled=move || format!("{}", disabled.get())
            on:click=move |e| {
                e.stop_propagation();
                on_click(e);
            }
        >

            {children()}
        </button>
    }
}

#[component]
pub fn LinkButton(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    href: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            id=id
            // format!("{}", )
            href=href
            class=format!("{} {}", variant.get(), class.get())
            style=style
            aria-disabled=move || disabled.get()
        >
            {children()}
        </a>
    }
}
