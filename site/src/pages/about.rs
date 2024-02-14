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
        <Main id="main">
            // class="max-w-[45rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto text-gray-900 dark:text-gray-400"
            <div class="flex flex-col pl-3 max-w-[45rem] 2xl:h-screen landscape:h-full px-4 py-4 sm:px-6 lg:px-8 mx-auto text-gray-900 dark:text-gray-400">
                // <!-- Announcement Banner -->
                <Typography variant=TypographyVariant::H1 class=TypographyClass::H1.get()>
                    Про Коваль
                </Typography>
                <AboutTextBlock/>
            // <!-- End Announcement Banner -->
            </div>

        </Main>
    }
}

#[component]
fn AboutTextBlock() -> impl IntoView {
    view! {
        <section class="max-w-[70rem] grow py-4 mx-auto">
            <div class="border rounded-xl shadow-sm border-blue-600 dark:bg-slate-800 dark:border-slate-600 p-5 pt-2 mt-3 mb-20">
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Що таке
                    {"«Коваль»?"}
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://github.com/alardev/koval"
                        target="_blank"
                    >
                        Коваль
                    </a>
                    {"— це безкоштовний вебзастосунок з відкритим вихідним кодом, створений для транслітерації з японської на українську, названий на честь"}
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://uk.wikipedia.org/wiki/%D0%9A%D0%BE%D0%B2%D0%B0%D0%BB%D0%B5%D0%BD%D0%BA%D0%BE_%D0%9E%D0%BB%D0%B5%D0%BA%D1%81%D0%B0%D0%BD%D0%B4%D1%80_%D0%9E%D0%BB%D0%B5%D0%BA%D1%81%D0%B0%D0%BD%D0%B4%D1%80%D0%BE%D0%B2%D0%B8%D1%87_(%D1%81%D1%85%D0%BE%D0%B4%D0%BE%D0%B7%D0%BD%D0%B0%D0%B2%D0%B5%D1%86%D1%8C)"
                        target="_blank"
                    >
                        Олександра Коваленка.
                    </a>

                </Typography>
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Навіщо він потрібен?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    {"Мета цього вебзастосунку — допомогти перекладачам стандартизувати та уточнити правильну транслітерацію імен, назв та інших слів. Однак він також може бути корисним для тих, хто вивчає мову."}
                </Typography>
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Як він працює?
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >

                    Сайт використовує модифіковану бібліотеку транслітератора
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://github.com/PSeitz/wana_kana_rust"
                        target="_blank"
                    >
                        wana_kana_rust,
                    </a>
                    яка спочатку використовувалася для транслітерації між катаканою, хіраганою та ромаджі.
                    <a
                        class="font-semibold dark:text-violet-400"
                        target="_blank"
                        href="https://github.com/alardev/wana_kana_rust"
                    >
                        Модифікація наведеної бібліотеки
                    </a>
                    допомогла створити транслітерацію хірагани/катакани українською мовою за використання
                    <a
                        class="font-semibold dark:text-violet-400"
                        target="_blank"
                        href="https://uk.wikipedia.org/wiki/%D0%AF%D0%BF%D0%BE%D0%BD%D1%81%D1%8C%D0%BA%D0%BE-%D1%83%D0%BA%D1%80%D0%B0%D1%97%D0%BD%D1%81%D1%8C%D0%BA%D1%96_%D1%81%D0%B8%D1%81%D1%82%D0%B5%D0%BC%D0%B8_%D1%82%D1%80%D0%B0%D0%BD%D1%81%D0%BA%D1%80%D0%B8%D0%BF%D1%86%D1%96%D1%97_%D1%82%D0%B0_%D1%82%D1%80%D0%B0%D0%BD%D1%81%D0%BB%D1%96%D1%82%D0%B5%D1%80%D0%B0%D1%86%D1%96%D1%97#%D0%9A%D0%BE%D0%B2%D0%B0%D0%BB%D0%B5%D0%BD%D0%BA%D0%BE_(2012)"
                    >
                        системи транслітерації Коваленка (2012).
                    </a>
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Для тестування використовувалася ручна транслітерація імен, що була створена
                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://t.me/HeiterkaEcchi"
                        target="_blank"
                    >
                        Гейтеркою Еччі,
                    </a>
                    а також роботи інших перекладачів й перекладачок. Без їхніх зусиль цей вебзастосунок не існував би!
                </Typography>
                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Важливо зазначити:
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    Усі можливі помилки є моїми власними й жодним чином не є результатом роботи будь-кого, на кого є посилання в цій роботі. Якщо ви знайшли будь-які помилки або маєте зауваження, будь ласка, повідомте про це через

                    <a
                        class="font-semibold dark:text-violet-400"
                        href="https://github.com/alardev/koval/issues/new"
                        target="_blank"
                    >
                        Github
                    </a>

                    або
                    {"зв'яжіться"}
                    зі мною у Телеграм. Також не соромтеся залишати відгуки, бо саме вони і є відзнакою й оцінкою моєї праці!

                </Typography>

                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Використані правила:
                </Typography>
                <Typography
                    variant=TypographyVariant::Paragraph
                    class=TypographyClass::Paragraph.get()
                >
                    <ul class="list-decimal ml-5">
                        <li>
                            {"Н/М: Перед складом на б, п, м ん \"n\" пишеться і читається як \"м\". ✅"}
                        </li>
                        <li>
                            {"Подвоєння приголосних (matcha — маччя)✅"}
                        </li>
                        <li>
                            Подовження голосних:
                            <ul class="list-disc ml-5">
                                <li>
                                    {"о: та у: Kyousuke — Кьоске ✅"}
                                </li>
                                <li>
                                    {"і: - За Коваленком, довга \"і:\" передається додаванням ї — \"ії\": Niigata — Ніїґата ✅"}
                                </li>
                                <li>
                                    {"а:, е: - зберігаємо: \"аа\", \"ее\". ✅"}
                                </li>
                                <li>
                                    {"розрізняти \"ou\" おう та \"oo\" おお.  ✅"}
                                </li>
                                <li>
                                    {"Не всі \"ou\" є подовженим \"о:\" - 井上 Inoue — Іноуе. ✅"}
                                </li>
                            </ul>
                        </li>
                        <li>
                            {"Дифтонги /ai, ii, ui, ei, oi/ - Nagai — Наґай ✅"}
                        </li>
                        <li>
                            {"Збіг голосних на межі складів - Hatsukaichi — Хацукаїчі ✅"}
                        </li>
                        <li>
                            {"Особливості апострофа - Shin'ichi — Шін'їчі (не Шінічі) ✅"}
                            <br/>
                            {"NB! треба додавати позначення 「 в хірагані/катакані"}
                        </li>
                        <li>
                            {"Сполучення  /ei/ : sensei — сенсей ✅"}
                        </li>
                        <li>
                            {"Редукція - Sasuke — Саске НЕ ВИКОРИСТОВУЄТЬСЯ 🔴"}
                        </li>
                        <li>
                            {"Є у Коваленка: Ieyasu — Ієясу ✅"}
                        </li>
                    </ul>
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
                            Додати можливість транслітерувати Канджі через бібліотеку jmdict
                        </li>
                        <li>
                            Створити базу імен персонажів та інших назв для зручного пошуку
                        </li>
                    </ul>
                </Typography>

                <Typography variant=TypographyVariant::H2 class=TypographyClass::H2.get()>
                    Окрема подяка:
                </Typography>
                <Typography
                variant=TypographyVariant::Paragraph
                class=TypographyClass::Paragraph.get()
                >
                    {"Наступні люди надали зворотній зв'язок про вебзастосунок на ранніх етапах та допомогли зробити його таким, яким він є сьогодні:"}
                    <ul class="list-disc ml-4 mt-4">
                        <li>
                            ОКЕ - Особлива подяка за ідею, допомогу з перекладами та правописом.
                        </li>
                        <li>
                            <a
                            class="font-semibold dark:text-violet-400"
                            href="https://www.instagram.com/stoka100k/"
                            target="_blank"
                            >
                                Сашко
                            </a> - За створення чудового логотипу для сайту.
                        </li>
                    </ul>
                </Typography>
                </div>
        </section>
    }
}
