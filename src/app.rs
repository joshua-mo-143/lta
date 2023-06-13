use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="../input.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {

    let (value, set_value) = create_signal(cx, 0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! { cx,
        <Title text="Leptos + Tailwindcss"/>
            <main>
                <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                    <div class="flex flex-row-reverse flex-wrap m-auto">
                        <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                            "+"
                        </button>
                        <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                            {value}
                        </button>
                        <button on:click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                            "-"
                        </button>
                    </div>
                </div>
            </main>
    }
}
