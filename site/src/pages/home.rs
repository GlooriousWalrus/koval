use leptos::{*, svg::view};
use wana_kana::ConvertJapanese;
use std::time::Duration;

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
pub fn Home() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal("".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let (hide_modal, set_hide_modal) = create_signal(true);


    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <main>
            <div class="bg-gradient-to-tl from-purple-800 
            to-violet-500 text-white font-mono 
            flex flex-col h-screen">                  

                <Toast hide_modal/>
                <div class="flex flex-col flex-nowrap 
                            self-center my-auto mx-6
                            w-2/5
                            sm:max-w-md md:max-w-lg lg:max-w-xl 
                            xl:max-w-2xl"
                >
                    <Outputfield name set_hide_modal/>
                    <p 
                        class="text-lg w-full p-2.5"
                    >
                            {"Транслітерація імені"}
                    </p>
                    <Inputfield name set_name input_element/>
                    <Versionwarning />
                </div>
            </div>
        </main>
    }
}


#[component]
fn Versionwarning() -> impl IntoView {
    let (hide_flag, set_hide_flag) = create_signal(false);

    view! {
        <div class="flex flex-col p-4 bg-purple-800 shadow-md hover:shadow-lg rounded-2xl grow" class:hidden=hide_flag>
        <div class="flex items-center justify-between">
            <div class="flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg"
                    class="w-24 h-14 md:w-16 md:h-16 rounded-2xl py-2 px-2 border border-gray-800 text-blue-400 bg-gray-900" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
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
                    <svg xmlns="http://www.w3.org/2000/svg" class="fill-current text-white-700" viewBox="0 0 16 16" width="20" height="20"><path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path></svg>
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
                    data-darkreader-inline-stroke="" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="M4.5 12.75l6 6 9-13.5" stroke-linecap="round" stroke-linejoin="round"></path>
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
fn Outputfield(name: ReadSignal<String>, set_hide_modal: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="bg-gray-50 border border-gray-300 
            text-gray-900 text-sm rounded-lg focus:ring-blue-500 
            focus:border-blue-500 block grow p-2.5 
            dark:bg-gray-700 dark:border-gray-600 
            dark:placeholder-gray-400 dark:text-white 
            dark:focus:ring-blue-500 
            dark:focus:border-blue-500
            shadow-lg overflow-scroll flex h-32
            no-scrollbar overflow-y-auto" 
        >
            <p 
                class="text-3xl p-2.5 grow text-center self-center"
            >
                    {name}
            </p>
            <button 
                class=
                    "bg-gray-50 border border-gray-300 
                    text-gray-900 text-sm rounded-lg focus:ring-blue-500 
                    focus:border-blue-500 block p-2.5 
                    dark:bg-gray-700 dark:border-gray-600 
                    dark:placeholder-gray-400 dark:text-white 
                    dark:focus:ring-blue-500 
                    dark:focus:border-blue-500
                    shadow-lg h-12 w-12 my-auto
                    transition dark:hover:bg-gray-600"
                on:click= move |evt: MouseEvent| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    copy_selected_text_to_clipboard(evt, name.get());

                    set_hide_modal(false);
                    set_timeout(
                        move || {
                            set_hide_modal(true)
                        },
                        Duration::new(2, 0),
                    )
                }
            >
                <svg 
                    data-darkreader-inline-stroke="" 
                    aria-hidden="true" 
                    fill="none" 
                    stroke="currentColor" 
                    stroke-width="1.5" 
                    viewBox="0 0 24 24" 
                    xmlns="http://www.w3.org/2000/svg">
                    <path d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
            </button>
        </div>
    }
}


#[component]
fn Inputfield(name: ReadSignal<String>, set_name: WriteSignal<String>, input_element: NodeRef<html::Input>) -> impl IntoView {
    view! {
        <input 
            type="text" 
            value=name
            on:input=move |ev| {
                ev.prevent_default();

                // here, we'll extract the value from the input
                let value = input_element()
                    // event handlers can only fire after the view
                    // is mounted to the DOM, so the `NodeRef` will be `Some`
                    .expect("<input> to exist")
                    // `NodeRef` implements `Deref` for the DOM element type
                    // this means we can call`HtmlInputElement::value()`
                    // to get the current value of the input
                    .value();

                set_name(value.to_romaji());
                // set_name(event_target_value(&ev));
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
            shadow-lg mb-5 min-w-fit" 
            name="title"
            placeholder="Введіть ім'я..."
        />
    }
}