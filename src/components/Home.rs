use leptos::{component, view, IntoView};
use crate::components::resume::ResumePage;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="text-red-800 text-center h-screen font-thin w-screen flex justify-between">
            <h1 class="h-full w-full bg-slate-400">"Welcome to Leptos!"</h1>
            <h1 class="h-full w-full bg-slate-500">"Welcome to Leptos!"</h1>
        </div>
        <section id="about" class="h-screen">
            <ResumePage/>
        </section>
    }
}