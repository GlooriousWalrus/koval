use crate::{components::variants::base::ClassVariant, OptionMaybeSignal};
use leptos::*;

#[component]
pub fn Container(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <div id=id class=format!("{} {}", variant.get(), class.get()) style=style>
            {children()}
        </div>
    }
}

#[component]
pub fn ContainerFromProp<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_view: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { <>{render_view()}</> }
}

#[component]
pub fn Main(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <main id=id role="main" class=class style=style>
            {children()}
        </main>
    }
}
