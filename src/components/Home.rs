use leptos::*;
use leptos::logging::log;
use std::thread::Scope;
use crate::components::resume::ResumePage;
use web_sys::HtmlDetailsElement;

#[component]
pub fn HomePage<'a>(cx: Scope<'a, 'a>) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-between h-screen font-thin w-screen py-5">
            <div class="text-white/90 flex flex-col lg:flex-row justify-between px-5 lg:px-10 items-center">
                <h1 class="bg-red-700 text-black p-3 rounded-sm mb-5 lg:mb-0">"Welcome to Leptos!"</h1>
                <div class="flex flex-row justify-between space-x-2 lg:space-x-4 w-full lg:w-1/4">
                    <h1 class="cursor-pointer">"LinkedIn"</h1>
                    <h1 class="cursor-pointer">"GitHub"</h1>
                    <h1 class="cursor-pointer">"Twitter"</h1>
                    <h1 class="cursor-pointer">"Docs"</h1>
                </div>
            </div>

            <div class="text-white/90 flex flex-col lg:flex-row justify-between px-5 lg:px-10 items-end">
                <h1 class="mb-5 lg:mb-0">"Welcome to Leptos!"</h1>
                <div class="flex flex-col justify-between">
                    <h1 class="cursor-pointer text-sm mb-1">"EDUCATION:"</h1>
                    <div class="flex flex-wrap space-x-1">
                        <h1 class="cursor-pointer border-[0.25px] border-gray-500 p-1 text-sm">"MS in CS"</h1>
                        <h1 class="cursor-pointer border-[0.25px] border-gray-500 p-1 text-sm">"CSU CHICO'24"</h1>
                        <h1 class="cursor-pointer border-[0.25px] border-gray-500 p-1 text-sm">"USA"</h1>
                    </div>
                </div>
                <h1 class="mt-5 lg:mt-0">"Welcome to Leptos!"</h1>
            </div>

            <DetailsTest cx/>
        </div>

        <section id="resume" class="h-screen w-screen">
            <ResumePage/>
        </section>
    }
}

#[component]
fn DetailsTest<'a>(cx: Scope<'a, 'a>) -> impl IntoView {
    let (open, set_open) = create_signal(false);

    let toggle_menu = move |_| {
        set_open(!open.get());
    };
    
    view! {
        <details open=open on:toggle=move|e| {
            log!("on toggle!");
            set_open(event_target::<HtmlDetailsElement>(&e).open());
        }>
            <summary>
                <button on:click=toggle_menu>
                    <svg width="23" height="18" viewBox="0 0 23 18">
                        <path fill="transparent"
                            stroke-width="3"
                            stroke="white"
                            stroke-linecap="round"
                            d={if open.get() { "M 3 16.5 L 17 2.5" } else { "M 2 2.5 L 20 2.5" }}
                            class="top" />
                        <path fill="transparent"
                            stroke-width="3"
                            stroke="white"
                            stroke-linecap="round"
                            d="M 6 9.423 L 20 9.423"
                            class="middle" />
                        <path fill="transparent"
                            stroke-width="3"
                            stroke="white"
                            stroke-linecap="round"
                            d={if open.get() { "M 3 2.5 L 17 16.346" } else { "M 2 16.346 L 20 16.346" }}
                            class="bottom" />
                    </svg>
                </button>
            </summary>
        </details>
    }
}