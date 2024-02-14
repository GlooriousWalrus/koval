use chrono::prelude::*;

use leptos::{logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_tw_ui::components::{
    buttons::button::LinkButton,
    menu::bars::{MenuBar, MenuHeader},
    theme::toggle::{theme_mode, MenuToggleButton, ThemeToggleSwitch},
};

use crate::{
    pages::{about::AboutPage, transliterator::Transliterator},
    theme::{
        default_page_class, ButtonVariant, MenuBarVariant, MenuHeaderVariant,
        ToggleSwitchClassVariant,
    },
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let formatter = |text| format!("{text} — Транслітерація японських імен");

    view! {
        <Layout>
            <Title formatter/>
            <Meta charset="utf-8"/>
            <Meta
                name="description"
                content="Транслітерація японських імен українською за стандартом Коваленко 2012. Транслітерує хіраґану та катакану. Для перекладачів аніме/мангі."
            />
            // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            <Meta name="author" content="alardev"/>
            <Link rel="shortcut icon" type_="image/svg+xml" href="/favicon.svg"/>
            <Router>
                <Routes>
                    <Route path="/" view=move || view! { <Transliterator/> }/>
                    <Route path="/about" view=move || view! { <AboutPage/> }/>
                </Routes>
            </Router>
        </Layout>
    }
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <LayoutWrapper>
            <Menu/>
            {children()}
            <Footer/>
        </LayoutWrapper>
    }
}

#[component]
fn LayoutWrapper(children: Children) -> impl IntoView {
    let default_class = default_page_class();

    view! {
        <div class=default_class.wrapper class:min-h-screen=true>
            {children()}
        </div>
    }
}

#[component]
fn Menu() -> impl IntoView {
    let (active, set_active) = create_signal(false);
    log!("loading Menu");

    view! {
        <MenuHeader variant=MenuHeaderVariant::Default.get()>
            <MenuBar variant=MenuBarVariant::Default.get()>
                <div class="flex items-center justify-between">
                    
                    <span class="select-none flex-none font-weight-20 text-3xl text-violet-800 dark:text-gray-200">
                        Коваль
                        <span class="text-sm block">
                            {"Транслітерація японських імен"}
                        </span>
                    </span>
                    <div class="sm:hidden">
                        <MenuToggleButton
                            on_change=set_active
                            class="focus:outline-none p-1"
                            icon_class="w-7 h-7 fill-gray-700 hover:fill-gray-500 dark:fill-gray-300 dark:hover:fill-gray-100 dark:hover:outline rounded-2xl"
                        />
                    </div>
                </div>
                <div class=move || match active() {
                    true => "transition-all duration-300 basis-full grow sm:block",
                    false => "hidden transition-all duration-300 basis-full grow sm:block",
                }>
                    <div class="select-none flex flex-col mt-4 sm:flex-row sm:items-center sm:justify-end sm:mt-0 sm:pl-5 ">
                        <LinkButton href="/" variant=ButtonVariant::Ghost.get()>
                            Транслітерація
                        </LinkButton>
                        <LinkButton href="/about" variant=ButtonVariant::Ghost.get()>
                            Про Коваль
                        </LinkButton>
                        <ThemeToggleSwitch
                            mode_fn=theme_mode
                            class=ToggleSwitchClassVariant::Encased.get()
                        />
                    </div>
                </div>
            </MenuBar>
        </MenuHeader>
    }
}

// Sticky footer
#[component]
fn Footer() -> impl IntoView {
    let year = Local::now().year();

    view! {
        <footer class="w-full fixed inset-x-0 z-50 bottom-0 p-4 bg-white border-t border-gray-200 shadow md:flex md:items-center md:justify-between md:p-6 dark:bg-slate-900 dark:border-gray-600">
            <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">
                {format!("© {} ", year)}
                <a href="https://github.com/alardev" class="hover:underline">
                    {"alardev"}
                </a>
            </span>
        // <ul class="flex flex-wrap items-center mt-3 text-sm font-medium text-gray-500 dark:text-gray-400 sm:mt-0">
        // <li>
        // <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
        // About
        // </LinkButton>
        // </li>
        // <li>
        // <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
        // Privacy Policy
        // </LinkButton>
        // </li>
        // <li>
        // <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
        // Licensing
        // </LinkButton>
        // </li>
        // <li>
        // <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
        // Contact
        // </LinkButton>

        // </li>
        // </ul>
        </footer>
    }
}
