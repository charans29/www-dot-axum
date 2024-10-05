use leptos::{component, view, IntoView};
use crate::components::resume::ResumePage;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="flex flex-col justify-between h-screen font-thin w-screen py-10">
            <div class="text-white/90 flex justify-between px-10">
                <h1>"Welcome to Leptos!"</h1>
                <h1>"Welcome to Leptos!"</h1>
            </div>
            <div class="text-white/90 flex justify-between px-10">
                <h1>"Welcome to Leptos!"</h1>
                <h1>"Welcome to Leptos!"</h1>
            </div>
        </div>
        <section id="resume" class="h-screen w-screen">
            <ResumePage/>
        </section>
    }
}