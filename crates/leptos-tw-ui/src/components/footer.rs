use leptos::*;

#[component]
pub fn Footer<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
    <footer className="py-6 md:px-8 md:py-0">
        {render_prop()}
        {children()}
    </footer>
    }
}
