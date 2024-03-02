use leptos::*;
use leptos_meta::Title;
use std::time::Duration;
use wana_kana::{ConvertJapanese, Options};

extern crate wana_kana;

use web_sys::MouseEvent;

pub fn copy_selected_text_to_clipboard(el: MouseEvent, content: String) {
    log::warn!("Copying text to clipboard..");

    let transforming_class =
        use_context::<ReadSignal<String>>().expect("to have found the getter provided");

    let text = match transforming_class.get().as_str() {
            "capitalize" => uppercase_words(&content), // Convert to String for consistency
            "uppercase" => content.to_uppercase(), // Returns a String, so it's owned
            "lowercase" => content.to_lowercase(), // Returns a String
            "normal-case" => content,
            _ => "".to_string(),
        };
    
    #[cfg(web_sys_unstable_apis)]
    if let Some(clipboard) = window().navigator().clipboard() {
        clipboard.write_text(text);
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
            <div class="flex flex-col landscape:h-full
            px-4 py-4 sm:px-6 lg:px-8 mx-auto min-h-screen">
                <Toast hide_modal/>
                <div class="flex flex-col flex-nowrap
                self-center mx-auto min-w-[20rem] max-w-xs sm:max-w-2xl md:min-w-[42rem]">
                    <Versionwarning/>
                    <Outputfield name set_hide_modal/>
                    <p class="text-lg dark:text-gray-100 w-full p-2.5">
                        {"Транслітерація імені"}
                    </p>
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
            class="flex flex-col mb-5 p-4 border border-primarylight bg-violet-200 dark:bg-primarylight dark:border-secondarylight shadow-md hover:shadow-lg rounded-2xl grow"
            class:hidden=hide_flag
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="w-24 h-14 md:w-16 md:h-16 rounded-2xl py-2 px-2 border border-secondarylight text-primarylight bg-violet-300"
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
                        <div class="font-medium leading-none dark:text-gray-100">
                            <b>{"Примітка: "}</b>
                            {"Наразі підтримуються лише хіраґана та катакана!"}
                            <br/>
                            {"Для розділення складів канджі використовуйте символ '_', наприклад: "}
                            <b>{"あか_いざわ"}</b>
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
            class="absolute mt-36 self-center z-50 flex flex-col
            p-4 bg-violet-300 shadow-md
            border border-primarylight
            hover:shadow-lg rounded-2xl 
            m-auto transition
            hover:bg-violet-200"
            class:hidden=hide_modal
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <svg
                        data-darkreader-inline-stroke=""
                        aria-hidden="true"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
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
                        <div class="font-medium leading-none dark:text-black">
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
        <div class="bg-gray-50 border border-gray-300
        text-gray-900 text-sm rounded-lg focus:ring-blue-500 
        focus:border-blue-500 block grow p-2.5 
        dark:bg-gray-700 dark:border-gray-600 
        dark:placeholder-gray-400 dark:text-white 
        dark:focus:ring-blue-500 
        dark:focus:border-blue-500
        shadow-lg overflow-scroll flex h-32
        no-scrollbar overflow-y-auto relative">

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
                transition dark:hover:bg-gray-600 min-w-[3rem] absolute inset-y-0 right-3"
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
                set_name(value.
                    to_ukrainian_with_opt(
                        Options {
                            imemode: true,
                            ..Default::default()
                        }
                    )
                )
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
fn SettingsSection() -> impl IntoView {
    let (settings_open, set_settings_open) = create_signal(false);

    // let message = move || settings_open().then(||
    //     view! { <CaseSection/> }
    // );

    view! { <CaseSection/> }
}

#[component]
fn Name(name: ReadSignal<String>) -> impl IntoView {
    let transforming_class =
        use_context::<ReadSignal<String>>().expect("to have found the getter provided");

    view! {
        <p class=move || {
            format!(
                "text-3xl select-all p-2.5
                text-center self-center
                overflow-y-auto w-11/12 {}",
                transforming_class.get(),
            )
        }>
            {
                move || {
                    match transforming_class.get().as_str() {
                        "capitalize" => uppercase_words(&name.get()), // Convert to String for consistency
                        "uppercase" => name.get().to_uppercase(), // Returns a String, so it's owned
                        "lowercase" => name.get().to_lowercase(), // Returns a String
                        "normal-case" => name.get(),
                        _ => "".to_string(),
                    }
                }
            }
        </p>
    }
}

#[component]
fn CaseSection() -> impl IntoView {
    let setter = use_context::<WriteSignal<String>>().expect("to have found the setter provided");

    let getter = use_context::<ReadSignal<String>>().expect("to have found the getter provided");

    view! {
        <div class="grid justify-items-center mb-20">
            <h2 class="p-2.5 text-lg dark:text-gray-100 max-w-fit">Регістр</h2>
            <div

                // absolute left-0 mt-2
                // z-10 origin-top-right

                class="
                bg-gray-50 border border-gray-300
                text-gray-900 text-sm rounded-lg focus:ring-blue-500 
                focus:border-blue-500
                dark:bg-gray-700 dark:border-gray-600 
                dark:placeholder-gray-400 dark:text-white 
                dark:focus:ring-blue-500 
                dark:focus:border-blue-500
                shadow-lg transition max-w-fit"
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="menu-button"
                tabindex="-1"
            >
                <div class="space-y-2 p-5">
                    <div class="relative flex w-56 items-center justify-center rounded-full bg-gray-50 px-4 py-3 font-medium text-gray-700">
                        <input
                            class="peer hidden"
                            type="radio"
                            name="frameworkB"
                            id="frameworkB1"
                            on:click=move |_| setter.set("capitalize".to_string())
                            checked=move || getter.get() == "capitalize"
                        />
                        <label
                            class="dark:bg-gray-600 peer-checked:border-primarylight peer-checked:bg-violet-200 dark:peer-checked:bg-violet-400 hover:bg-violet-100 peer-checked:hover:bg-violet-200 absolute top-0 h-full w-full cursor-pointer rounded-full border"
                            for="frameworkB1"
                        ></label>
                        <div class="peer-checked:border-transparent peer-checked:bg-primarylight peer-checked:ring-2 absolute left-4 h-5 w-5 rounded-full border-2 border-gray-300 bg-gray-200 ring-primarylight ring-offset-2"></div>
                        <span class="dark:text-gray-50 peer-checked:text-black pointer-events-none z-10">
                            З великої літери
                        </span>
                    </div>
                    <div class="relative flex w-56 items-center justify-center rounded-full bg-gray-50 px-4 py-3 font-medium text-gray-700">
                        <input
                            class="peer hidden"
                            type="radio"
                            name="frameworkB"
                            id="frameworkB2"
                            on:click=move |_| setter.set("uppercase".to_string())
                            checked=move || getter.get() == "uppercase"
                        />
                        <label
                            class="dark:bg-gray-600 peer-checked:border-primarylight peer-checked:bg-violet-200 dark:peer-checked:bg-violet-400 hover:bg-violet-100 peer-checked:hover:bg-violet-200 absolute top-0 h-full w-full cursor-pointer rounded-full border"
                            for="frameworkB2"
                        ></label>
                        <div class="peer-checked:border-transparent peer-checked:bg-primarylight peer-checked:ring-2 absolute left-4 h-5 w-5 rounded-full border-2 border-gray-300 bg-gray-200 ring-primarylight ring-offset-2"></div>
                        <span class="dark:text-gray-50 peer-checked:text-black pointer-events-none z-10">
                            Верхній регістр
                        </span>
                    </div>
                    <div class="relative flex w-56 items-center justify-center rounded-full bg-gray-50 px-4 py-3 font-medium text-gray-700">
                        <input
                            class="peer hidden"
                            type="radio"
                            name="frameworkB"
                            id="frameworkB3"
                            on:click=move |_| setter.set("lowercase".to_string())
                            checked=move || getter.get() == "lowercase"
                        />
                        <label
                            class="dark:bg-gray-600 peer-checked:border-primarylight peer-checked:bg-violet-200 dark:peer-checked:bg-violet-400 hover:bg-violet-100 peer-checked:hover:bg-violet-200 absolute top-0 h-full w-full cursor-pointer rounded-full border"
                            for="frameworkB3"
                        ></label>
                        <div class="peer-checked:border-transparent peer-checked:bg-primarylight peer-checked:ring-2 absolute left-4 h-5 w-5 rounded-full border-2 border-gray-300 bg-gray-200 ring-primarylight ring-offset-2"></div>
                        <span class="dark:text-gray-50 peer-checked:text-black pointer-events-none z-10">
                            Нижній регістр
                        </span>
                    </div>
                    <div class="relative flex w-56 items-center justify-center rounded-full bg-gray-50 px-4 py-3 font-medium text-gray-700">
                        <input
                            class="peer hidden"
                            type="radio"
                            name="frameworkB"
                            id="frameworkB4"
                            on:click=move |_| setter.set("normal-case".to_string())
                            checked=move || getter.get() == "normal-case"
                        />
                        <label
                            class="dark:bg-gray-600 peer-checked:border-primarylight peer-checked:bg-violet-200 dark:peer-checked:bg-violet-400 hover:bg-violet-100 peer-checked:hover:bg-violet-200 absolute top-0 h-full w-full cursor-pointer rounded-full border"
                            for="frameworkB4"
                        ></label>
                        <div class="peer-checked:border-transparent peer-checked:bg-primarylight peer-checked:ring-2 absolute left-4 h-5 w-5 rounded-full border-2 border-gray-300 bg-gray-200 ring-primarylight ring-offset-2"></div>
                        <span class="dark:text-gray-50 peer-checked:text-black pointer-events-none z-10">
                            Без форматування
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn uppercase_words(data: &str) -> String {
    data.split_whitespace() // Split the input string into an iterator of words.
        .map(|word| { // Map each word to a new string with the first letter capitalized.
            let mut chars = word.chars(); // Get an iterator of the characters in the word.
            match chars.next() { // Match on the first character of the word.
                Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(), // Return an empty string if there is no first character.
            }
        })
        .collect::<Vec<String>>() // Collect the mapped words into a vector of strings.
        .join(" ") // Join the vector of strings into a single string with spaces.
}
