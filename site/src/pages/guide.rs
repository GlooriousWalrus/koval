use leptos::*;
use leptos_meta::Title;
use leptos_tw_ui::components::{
    container::Main,
    typography::{Typography, TypographyVariant},
};

use crate::theme::TypographyClass;

#[component]
pub fn GuidePage() -> impl IntoView {
    view! {
        <Title text="Коваль | Посібник"/>
        <Main id="main">
            // class="max-w-[45rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto text-gray-900 dark:text-gray-400"
            <div class="flex flex-col pl-3 max-w-[45rem] 2xl:h-screen landscape:h-full px-4 py-4 sm:px-6 lg:px-8 mx-auto text-gray-900 dark:text-gray-400">
                // <!-- Announcement Banner -->
                <Typography variant=TypographyVariant::H1 class=TypographyClass::H1.get()>
                    Посібник
                </Typography>
                <GuideTextBlock/>
            // <!-- End Announcement Banner -->
            </div>

        </Main>
    }
}

#[component]
fn GuideTextBlock() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] grow py-4 mx-auto">
            <div class="border rounded-xl shadow-sm border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 pt-2 mt-3 mb-20">
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Як використовувати
                    {"«Коваль»?"}
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Для того, щоб Коваль міг транслітерувати японські імена, ви маєте вводити їх, використовуючи фонетичну систему письма
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://uk.wikipedia.org/wiki/%D0%A5%D1%96%D1%80%D0%B0%D2%91%D0%B0%D0%BD%D0%B0"
                        target="_blank"
                    >
                        хірагана
                    </a>
                    або
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://uk.wikipedia.org/wiki/%D0%9A%D0%B0%D1%82%D0%B0%D0%BA%D0%B0%D0%BD%D0%B0"
                        target="_blank"
                    >
                        катакана.
                    </a>

                    // <div>
                    //     <ruby class="text-5xl">赤<rp>{"（"}</rp><rt>あか</rt><rp>{"）"}</rp></ruby>
                    //     <ruby class="text-5xl">井<rp>{"（"}</rp><rt>い</rt><rp>{"）"}</rp></ruby>
                    //     <ruby class="text-5xl">沢<rp>{"（"}</rp><rt>ざわ</rt><rp>{"）"}</rp></ruby>
                    // </div>
                    <h3 class="my-3">
                        {"Наприклад:"}
                        <br/>
                        {"にほん - Ніхон."}
                        <br/>
                        {"しんいちろう - Шін'їчіро."}
                    </h3>

                    <h3 class="my-3">
                        <p class="font-bold text-xl">
                            {"Увага!"}
                        </p>
                        {"У деяких випадках потрібно використовувати спеціальний символ-розділювач \"_\" для позначення розриву між складами канджі. Прочитайте про це нижче, щоб зрозуміти, чому це важливо."}
                    </h3>

                    <h3 class="my-3">
                        {"Приклад застосування:"}
                        <br/>
                        {"はつか_いち - Хацукаїчі."}
                        <br/>
                        {"ごう あか_いざわ - Ґо Акаїдзава."}
                    </h3>
                </Typography>

                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Навіщо позначати розділення складів?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    {"Існують певні слова та імена як はつかいち. Читається як \"Хацукаїчі\" - місто в Японії. Однак, якщо ви введете це фонетичне читання у Коваль, він видасть ХацукаЙчі. Чому це так?"}
                    <p class="mt-3">
                        {"Причина в тому, що хіраґана і катакана - це фонетичні алфавіти, які використовуються для позначення вимови іншого набору японської писемності, який називається"}
                        
                        <a
                            class="font-semibold dark:text-violet-400"
                            href="https://uk.wikipedia.org/wiki/%D0%9A%D0%B0%D0%BD%D0%B4%D0%B7%D1%96"
                            target="_blank"
                        >
                            канджі.
                        </a>
                        
                        {"Багато слів і особливо імен пишуться використовуючи ієрогліфи канджі. Кожен ієрогліф канджі це склад. У системі транслітерації Коваленка, коли один склад канджі закінчується з голосним звуком, а інший канджі починається зі звуку I, це називається збігом, що означає, що буква I перетворюється на Ї."}
                    </p>

                    <p class="mt-3">
                        {"Давайте розглянемо канджі Хацукаїчі, щоб краще зрозуміти, чому важливо знати, де починається і де закінчується читання складів канджі."}
                    </p>
                    <div class="mt-3">
                        <ruby class="text-4xl">廿<rp>{"（"}</rp><rt>はつ</rt><rp>{"）"}</rp></ruby>
                        <ruby class="text-4xl">日<rp>{"（"}</rp><rt>か</rt><rp>{"）"}</rp></ruby>
                        <ruby class="text-4xl">市<rp>{"（"}</rp><rt>いち</rt><rp>{"）"}</rp></ruby>
                    </div>

                    <p class="mt-3">
                        {"Саме так записується ім'я Хацукаїчі у вигляді канджі. Наступне позначення, яке ви бачите, називається"} 
                        
                        <a
                            class="font-semibold dark:text-violet-400"
                            href="https://uk.wikipedia.org/wiki/%D0%A4%D1%83%D1%80%D1%96%D2%91%D0%B0%D0%BD%D0%B0"
                            target="_blank"
                        >
                            Фуріґана. 
                        </a>

                        
                        
                        {"Фуріґана використовується для написання канджі з відповідним читанням над ними. Ви дуже часто зустрінете це в японській літературі та медіа."}
                    </p>

                    <p class="mt-3">
                        {"Тепер ми можемо записати назву наступним чином: はつ_か_いち. Тепер, коли ми вводимо це у Коваль, ми отримуємо правильний очікуваний результат Хацукаїчі."}
                    </p>

                    <p class="mt-3">
                        {"Ви могли помітити, що ієрогліф 廿 читається як はつ (хацу українською). В японській мові канджі може мати довільне читання, яке автор бажає. Вони можуть буквально придумати власне значення, якщо захочуть. Один ієрогліф канджі може означати багато різних речей і може бути вимовлений або прочитаний кількома абсолютно різними способами. Отже, використовуючи лише канджі, неможливо визначити вимову. Коли ви зустрічаєте японця або бачите ім'я, яке написане у вигляді канджі, ви можете інтуїтивно здогадатися, як правильно вимовляти, але щоб бути абсолютно впевненим, ви повинні запитати людину, як воно вимовляється."}
                    </p>

                    <p class="mt-3">
                        {"Саме тому вам потрібно знати, як кожне ім'я пишеться у формі канджі, і як вимовляється кожен склад."}
                    </p>
                    <p class="mt-3 font-bold">
                        {"Просто використовувати канджі недостатньо. Тому треба використовувати хірагану і вручну відзначати розділення."}
                    </p>
                </Typography>

                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Де знайти вимову іменам в хірагани та катакани?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    <p class="mb-3">
                        {"Перше місце, де ви повинні шукати значення канджі, - це першоджерело, з яким ви працюєте. Автор, найімовірніше, зазначить нестандартне прочитання будь-яких імен чи слів, які він вважатиме за потрібне."}
                    </p>

                    <p class="my-3">
                        {"Крім того, ви можете спробувати пошукати в Інтернеті або скористатися такими інструментами, як"}
                    </p>
                    <ul class="list-disc ml-4 mt-4">
                        <li>
                            <a
                                class="font-semibold dark:text-violet-400"
                                href="https://jotoba.de"
                                target="_blank"
                            >
                                jotoba.de
                            </a>
                        </li>
                    </ul>
                </Typography>
            </div>
        </section>
    }
}
