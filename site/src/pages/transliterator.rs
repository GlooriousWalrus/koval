use leptos::*;
use leptos_meta::Title;
use std::time::Duration;
use wana_kana::ConvertJapanese;

extern crate wana_kana;

use web_sys::MouseEvent;

pub fn copy_selected_text_to_clipboard(el: MouseEvent, content: String) {
    log::warn!("Copying text to clipboard..");
    #[cfg(web_sys_unstable_apis)]
    if let Some(clipboard) = window().navigator().clipboard() {
        clipboard.write_text(&content);
    } else {
        log::error!("Clipboard is not supported");
    }
}

#[component]
pub fn Transliterator() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal("".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let (hide_modal, set_hide_modal) = create_signal(true);

    let (transforming_class, set_transforming_class) = create_signal("capitalize".to_string());

    provide_context(set_transforming_class);
    provide_context(transforming_class);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Коваль | Транслітератор"/>
        <main>
            <div class="flex flex-col max-w-[85rem] h-screen 
            px-4 py-4 sm:px-6 lg:px-8 mx-auto ">
                <Toast hide_modal/>
                <div class="flex flex-col flex-nowrap
                self-center my-auto mx-6">
                    <Versionwarning/>
                    <Outputfield name set_hide_modal/>
                    <p class="text-lg dark:text-gray-100 w-full p-2.5">{"Транслітерація імені"}</p>
                    <Inputfield name set_name input_element/>
                    <SettingsSection/>
                </div>
            </div>
        </main>
    }
}

#[component]
fn Versionwarning() -> impl IntoView {
    let (hide_flag, set_hide_flag) = create_signal(false);

    view! {
        <div
            class="flex flex-col mb-5 p-4 bg-purple-800 shadow-md hover:shadow-lg rounded-2xl grow"
            class:hidden=hide_flag
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="w-24 h-14 md:w-16 md:h-16 rounded-2xl py-2 px-2 border border-gray-800 text-blue-400 bg-gray-900"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        ></path>
                    </svg>
                    <div class="flex flex-col ml-3">
                        <div class="font-medium leading-none text-gray-100">
                            <b>{"Примітка: "}</b>
                            {"Наразі підтримуються лише хіраґана та катакана!"}
                        </div>
                    </div>
                    <button
                        class="md:ml-6"
                        on:click=move |_| {
                            set_hide_flag(true);
                        }
                    >

                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="fill-current text-white-700"
                            viewBox="0 0 16 16"
                            width="20"
                            height="20"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"
                            ></path>
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Toast(hide_modal: ReadSignal<bool>) -> impl IntoView {
    view! {
        <button
            type="button"
            class="fixed self-center
            mt-12 z-50 flex flex-col 
            p-4 bg-purple-800 shadow-md 
            hover:shadow-lg rounded-2xl 
            m-auto transition 
            hover:bg-purple-600"
            class:hidden=hide_modal
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <svg
                        data-darkreader-inline-stroke=""
                        aria-hidden="true"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.5"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M4.5 12.75l6 6 9-13.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></path>
                    </svg>
                    <div class="flex flex-col ml-3">
                        <div class="font-medium leading-none text-gray-100">
                            <b>{"Ім'я скопійовано"}</b>
                        </div>
                    </div>
                </div>
            </div>
        </button>
    }
}

#[component]
fn Outputfield(
    name: ReadSignal<String>,
    set_hide_modal: WriteSignal<bool>,
) -> impl IntoView {

    view! {
        <div class="bg-gray-50 border border-gray-300
        text-gray-900 text-sm rounded-lg focus:ring-blue-500 
        focus:border-blue-500 block grow p-2.5 
        dark:bg-gray-700 dark:border-gray-600 
        dark:placeholder-gray-400 dark:text-white 
        dark:focus:ring-blue-500 
        dark:focus:border-blue-500
        shadow-lg overflow-scroll flex h-32
        no-scrollbar overflow-y-auto">

            <Name name/>

            <button
                class="bg-gray-50 border border-gray-300
                text-gray-900 text-sm rounded-lg focus:ring-blue-500 
                focus:border-blue-500 block p-2.5 
                dark:bg-gray-700 dark:border-gray-600 
                dark:placeholder-gray-400 dark:text-white 
                dark:focus:ring-blue-500 
                dark:focus:border-blue-500
                shadow-lg h-12 w-12 my-auto
                transition dark:hover:bg-gray-600"
                on:click=move |evt: MouseEvent| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    copy_selected_text_to_clipboard(evt, name.get());
                    set_hide_modal(false);
                    set_timeout(move || { set_hide_modal(true) }, Duration::new(2, 0))
                }
            >

                <svg
                    data-darkreader-inline-stroke=""
                    aria-hidden="true"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </svg>
            </button>
        </div>
    }
}

#[component]
fn Inputfield(
    name: ReadSignal<String>,
    set_name: WriteSignal<String>,
    input_element: NodeRef<html::Input>,
) -> impl IntoView {
    view! {
        <input
            type="text"
            value=name
            on:input=move |ev| {
                ev.prevent_default();
                let value = input_element().expect("<input> to exist").value();
                set_name(value.to_romaji());
            }

            // prop:value=name
            node_ref=input_element
            class="bg-gray-50 border border-gray-300
            text-gray-900 text-sm rounded-lg focus:ring-blue-500 
            focus:border-blue-500 block grow p-2.5 
            dark:bg-gray-700 dark:border-gray-600 
            dark:placeholder-gray-400 dark:text-white 
            dark:focus:ring-blue-500 
            dark:focus:border-blue-500
            shadow-lg min-w-fit"
            name="title"
            placeholder="Введіть ім'я..."
        />
    }
}

#[component]
fn SettingsSection(
) -> impl IntoView {

    let (settings_open, set_settings_open) = create_signal(false);

    let message = move || settings_open().then(|| 
        view! { <CaseSection/> }
    );

    view! {
        <div class="mt-5">
            <button
                class="bg-gray-50 border border-gray-300
                text-gray-900 text-sm rounded-lg focus:ring-blue-500 
                focus:border-blue-500 block p-2.5 
                dark:bg-gray-700 dark:border-gray-600 
                dark:placeholder-gray-400 dark:text-white 
                dark:focus:ring-blue-500 
                dark:focus:border-blue-500
                shadow-lg h-12 my-auto
                transition dark:hover:bg-gray-600
                flex flex-row items-center justify-center"
                on:click=move |evt: MouseEvent| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    set_settings_open(!settings_open());
                }
            >

                <svg
                    class="h-full w-2/5"
                    data-darkreader-inline-stroke=""
                    aria-hidden="true"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                    <path
                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </svg>
                <p class="text-center pl-2 py-0">{"Налаштування"}</p>
            </button>

            {message}
        </div>
    }
}

#[component]
fn Name(
    name: ReadSignal<String>, 
) -> impl IntoView {
    let transforming_class = use_context::<ReadSignal<String>>()
        .expect("to have found the getter provided");

    view! {
        <p class=move || {
            format!(
                "text-3xl select-all p-2.5 
                text-center w-11/12 self-center 
                overflow-y-auto {}",
                transforming_class.get(),
            )
        }>{name}</p>
    }
}

#[component]
fn CaseSection(

) -> impl IntoView {
    let setter = use_context::<WriteSignal<String>>()
        .expect("to have found the setter provided");

    let getter = use_context::<ReadSignal<String>>()
        .expect("to have found the getter provided");

    view! {
        <div class="relative">
            <div
                class="absolute left-0 mt-2 
                z-10 origin-top-right
                bg-gray-50 border border-gray-300
                text-gray-900 text-sm rounded-lg focus:ring-blue-500 
                focus:border-blue-500
                dark:bg-gray-700 dark:border-gray-600 
                dark:placeholder-gray-400 dark:text-white 
                dark:focus:ring-blue-500 
                dark:focus:border-blue-500
                shadow-lg transition"
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="menu-button"
                tabindex="-1"
            >
                <div class="first:rounded-t-lg last:rounded-b-lg" role="none">
                    // Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700"
                    <button
                        class="block px-4 py-2 w-full rounded-t-lg hover:bg-gray-400 active:hover:bg-blue-400 hover:active:bg-blue-400"
                        role="menuitem"
                        tabindex="-1"
                        id="menu-item-0"
                        on:click=move |_| setter.set("capitalize".to_string())
                        class:bg-blue-400=move || getter.get() == "capitalize"
                    >
                        З великої літери
                    </button>
                    <button
                        class="block px-4 py-2 w-full hover:bg-gray-400 hover:active:bg-blue-400"
                        role="menuitem"
                        tabindex="-1"
                        id="menu-item-1"
                        on:click=move |_| setter.set("uppercase".to_string())
                        class:bg-blue-400=move || getter.get() == "uppercase"
                    >
                        Верхній регістр
                    </button>
                    <button
                        class="block px-4 py-2 w-full rounded-b-lg hover:bg-gray-400 active:hover:bg-blue-400"
                        role="menuitem"
                        tabindex="-1"
                        id="menu-item-2"
                        on:click=move |_| setter.set("lowercase".to_string())
                        class:bg-blue-400=move || getter.get() == "lowercase"
                    >
                        Нижній регістр
                    </button>
                // <form method="POST" action="#" role="none">
                // <button type="submit" class="text-gray-700 block w-full px-4 py-2 text-left text-sm" role="menuitem" tabindex="-1" id="menu-item-3">Sign out</button>
                // </form>
                </div>
            </div>
        </div>
    }
}