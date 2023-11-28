use leptos::*;
use leptos_meta::Title;
use leptos_tw_ui::components::{
    container::Main,
    typography::{Typography, TypographyVariant},
};

use crate::theme::TypographyClass;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="Коваль | Про Коваль"/>
        <Main
            id="main"
            class="max-w-[45rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto text-gray-900 dark:text-gray-400"
        >
            <div class="pl-3">
                // <!-- Announcement Banner -->
                <Typography variant=TypographyVariant::H1 class=TypographyClass::H1.get()>
                    Про Коваль
                </Typography>
            // <!-- End Announcement Banner -->
            </div>
            <AboutTextBlock/>

        </Main>
    }
}

#[component]
fn AboutTextBlock() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] py-4 mx-auto">
            <div class="border rounded-xl shadow-sm border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 pt-2 mt-3">
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Що?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://github.com/GlooriousWalrus/koval"
                        target="_blank"
                    >
                        Коваль
                    </a>
                    - це безкоштовний вебзастосунок з відкритим вихідним кодом, створений для транслітерації з японської на українську, названий на честь
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://uk.wikipedia.org/wiki/%D0%9A%D0%BE%D0%B2%D0%B0%D0%BB%D0%B5%D0%BD%D0%BA%D0%BE_%D0%9E%D0%BB%D0%B5%D0%BA%D1%81%D0%B0%D0%BD%D0%B4%D1%80_%D0%9E%D0%BB%D0%B5%D0%BA%D1%81%D0%B0%D0%BD%D0%B4%D1%80%D0%BE%D0%B2%D0%B8%D1%87_(%D1%81%D1%85%D0%BE%D0%B4%D0%BE%D0%B7%D0%BD%D0%B0%D0%B2%D0%B5%D1%86%D1%8C)"
                        target="_blank"
                    >
                        Олександра Коваленка
                    </a>
                    .
                </Typography>
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Навіщо він потрібен?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Він був створений, щоб допомогти перекладачам стандартизувати та уточнити транслітерацію імен, назв та інших слів. Однак він також може бути корисним для тих, хто вивчає мову.
                </Typography>
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Як він працює?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Цей сайт використовує модифіковану бібліотеку транслітератора
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://github.com/PSeitz/wana_kana_rust"
                        target="_blank"
                    >
                        wana_kana_rust
                    </a>
                    , яка спочатку використовувалася для транслітерації між катаканою, хіраганою та ромадзі. Я модифікував ту бібліотеку, щоб спробувати конвертувати з хірагани/катакани в українську мову, використовуючи
                    <a
                        class="font-semibold dark:text-violet-400"
                        target="_blank"
                        href="https://uk.wikipedia.org/wiki/%D0%AF%D0%BF%D0%BE%D0%BD%D1%81%D1%8C%D0%BA%D0%BE-%D1%83%D0%BA%D1%80%D0%B0%D1%97%D0%BD%D1%81%D1%8C%D0%BA%D1%96_%D1%81%D0%B8%D1%81%D1%82%D0%B5%D0%BC%D0%B8_%D1%82%D1%80%D0%B0%D0%BD%D1%81%D0%BA%D1%80%D0%B8%D0%BF%D1%86%D1%96%D1%97_%D1%82%D0%B0_%D1%82%D1%80%D0%B0%D0%BD%D1%81%D0%BB%D1%96%D1%82%D0%B5%D1%80%D0%B0%D1%86%D1%96%D1%97#%D0%9A%D0%BE%D0%B2%D0%B0%D0%BB%D0%B5%D0%BD%D0%BA%D0%BE_(2012)"
                    >
                        стандарт Коваленко 2012
                    </a>
                    .
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Тестування проводилося шляхом порівняння зі списком імен персонажів з різних манґ та аніме, складеною
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://t.me/HeiterkaEcchi"
                        target="_blank"
                    >
                        Гейтеркою Еччі
                    </a>
                    та іншими перекладачами. Без їхніх зусиль цей вебзастосунок не існував би!
                </Typography>
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Зауваження
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Усі помилки є моїми власними й жодним чином не є результатом роботи будь-кого, на кого є посилання в цій роботі.
                    Якщо ви знайшли будь-які помилки або недоліки, будь ласка, повідомте мені про це через

                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://github.com/GlooriousWalrus/koval/issues/new"
                        target="_blank"
                    >
                        Github
                    </a>

                    або
                    {"зв'яжіться"}
                    зі мною в телеграмі. 
                    
                    Всі відгуки вітаються!

                </Typography>

                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Плани на майбутнє
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    <ul class="list-disc ml-4">
                        <li>
                            Додати можливість транслітерувати Кандзі через бібліотеку jmdict
                        </li>
                        <li>
                            Створити базу імен персонажів та інших назв для зручного пошуку
                        </li>
                    </ul>

                </Typography>
            </div>
        </section>
    }
}
