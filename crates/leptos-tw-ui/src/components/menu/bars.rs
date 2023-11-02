use crate::{components::variants::base::ClassVariant, OptionMaybeSignal};
use leptos::*;

#[component]
pub fn MenuBar(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav
            id=id
            class=format!("{} {}", variant.get(), class.get())
            style=style
        >
            { children() }
        </nav>
    }
}

#[component]
pub fn MenuHeader(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <header
            id=id
            class=format!("{} {}", variant.get(), class.get())
            style=style
        >
            { children() }
        </header>
    }
}
