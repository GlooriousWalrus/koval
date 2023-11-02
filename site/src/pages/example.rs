use leptos::*;
use leptos_meta::Title;
use leptos_tw_ui::components::{
    buttons::button::Button,
    container::{Container, Main},
    theme::toggle::{theme_mode, ThemeToggleButton, ThemeToggleSwitch},
    typography::{Typography, TypographyVariant},
};

use crate::theme::{ButtonVariant, ToggleSwitchClassVariant, TypographyClass};
use std::ops::Not;

#[component]
pub fn ExamplePage() -> impl IntoView {
    view! {
        <Title text="Leptos + Tailwindcss | Examples"/>
        <Main id="main" class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto pb-[100px] text-gray-900 dark:text-gray-400">
            <div>
                //   <!-- Announcement Banner -->
                <Typography variant=TypographyVariant::H1 class={TypographyClass::H1.get()}>Examples with Explanations</Typography>
                //   <!-- End Announcement Banner -->
            </div>
            <DarkModeSection/>
            <ButtonSection/>
            <TypographySection/>
        </Main>
        // An empty Fragment is not created on the DOM
        {Fragment::new(vec![])}
    }
}

#[component]
fn ButtonSection() -> impl IntoView {
    let (disabled, set_disabled) = create_signal(false);

    view! {
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <Typography variant=TypographyVariant::H2 class={TypographyClass::H2.get()}>Buttons</Typography>
            <div class="border rounded-xl shadow-sm  border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 mt-3">
                <Typography variant=TypographyVariant::H3 class={TypographyClass::H3.get()}>Inline Block</Typography>
                <div class="p-4 inline-block">
                    <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                </div>
                <Typography variant=TypographyVariant::H3 class={TypographyClass::H3.get()}>Grid (md:grid-cols-3)</Typography>
                <div class="p-4 grid md:grid-cols-3 gap-4">
                    <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                </div>
                <Typography variant=TypographyVariant::H3 class={TypographyClass::H3.get()}>Flex (flex-wrap)</Typography>
                <div class="p-4 flex flex-wrap gap-2">
                    <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                    <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                    <Button class="ml-0 mx-1" on_click=move |_| set_disabled(if !disabled.get_untracked() {true} else {false}) disabled=disabled variant={ButtonVariant::Solid.get()}>{move || if disabled.get() {"Disabled"} else {"Enabled"}}</Button>
                    <Button class="ml-0 mx-1" on_click=move |_| set_disabled.update(|value| *value = if *value {false} else {true}) disabled=disabled variant={ButtonVariant::Solid.get()}>{move || if disabled.get() {"Disabled"} else {"Enabled"}}</Button>
                    <Button class="ml-0 mx-1" on_click=move |_| set_disabled.update(|value| *value = value.not()) disabled=disabled variant={ButtonVariant::Solid.get()}>{move || if disabled.get() {"Disabled"} else {"Enabled"}}</Button>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TypographySection() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <Typography variant=TypographyVariant::H2 class={TypographyClass::H2.get()}>Typography</Typography>
            <div class="border rounded-xl shadow-sm  border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 mt-3">
                <Typography variant=TypographyVariant::H1 class={TypographyClass::H1.get()}>H1</Typography>
                <Typography variant=TypographyVariant::H2 class={TypographyClass::H2.get()}>H2</Typography>
                <Typography variant=TypographyVariant::H3 class={TypographyClass::H3.get()}>H3</Typography>
                <Typography variant=TypographyVariant::H4 class={TypographyClass::H4.get()}>H4</Typography>
                <Typography variant=TypographyVariant::H5 class={TypographyClass::H5.get()}>H5</Typography>
                <Typography variant=TypographyVariant::H6 class={TypographyClass::H6.get()}>H6</Typography>
                <Typography variant=TypographyVariant::Paragraph class={TypographyClass::Paragraph.get()}>This is a Paragraph</Typography>
                <p>The word <Typography variant=TypographyVariant::SpanInline class={TypographyClass::Span.get()}>span</Typography> in this sentence is wrapped,
                also the following code
                <Typography variant=TypographyVariant::Code {inline:true} class={TypographyClass::Code.get()}>This is a Code block</Typography>
                is wrapped.</p>
                <p>The word <Typography variant={TypographyVariant::Span {inline:false}} class={TypographyClass::Span.get()}>span</Typography> in this sentence is wrapped,
                also the following code
                <Typography variant=TypographyVariant::Code {inline:false} class={TypographyClass::Code.get()}>This is a Code block</Typography>
                is wrapped but they are not inline.</p>
                <Typography variant=TypographyVariant::Code {inline:false} class={TypographyClass::Code.get()}>
                    <Typography variant=TypographyVariant::Span {inline:false}>let num: i32 = 100;</Typography>
                    <Typography variant=TypographyVariant::Span {inline:false}>"println!{""{}"", num};"</Typography>
                </Typography>
            </div>
        </section>
    }
}

#[component]
fn DarkModeSection() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <Typography variant=TypographyVariant::H2 class={TypographyClass::H2.get()}>Dark/Light Theme Toggle</Typography>
            <Typography variant=TypographyVariant::Paragraph class={TypographyClass::Paragraph.get()}>
                There are currently two options for toggle component (ThemeToggleButton and ThemeToggleSwitch). All variants use an on click event to
                change the mode of the theme from dark to light. The preference is stored into local storage for persistence between sessions.
                Icon switching is not set by stored state, but by the theme setting using tailwindcss (hidden). Initial defaults to dark if there is a system preference.
            </Typography>
            <div class="border rounded-xl shadow-sm border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 mt-3">
                <Container class="py-2">
                    <ThemeToggleButton mode_fn={theme_mode} class="text-yellow-700 dark:text-primary-400 focus:outline-none text-sm p-1"
                        icon_light_class="dark:hidden w-9 h-9 fill-orange-300 hover:bg-yellow-200 rounded-2xl"
                        icon_dark_class="hidden dark:block w-9 h-9 fill-yellow-300 hover:fill-gray-800 hover:bg-yellow-300 rounded-2xl" />
                </Container>
                <Container class="py-2">
                    <ThemeToggleSwitch mode_fn={theme_mode} class={ToggleSwitchClassVariant::Knob.get()} />
                </Container>
                <Container class="py-2">
                    <ThemeToggleSwitch mode_fn={theme_mode} class={ToggleSwitchClassVariant::Encased.get()} />
                </Container>
            </div>
        </section>
    }
}
