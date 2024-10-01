use leptos::{component, view, IntoView};
use crate::components::resume::ResumePage;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="text-blue-600 text-center h-full py-14 font-thin w-screen">
            <h1>"Welcome to Leptos!"</h1>
        </div>
        <section id="about" class="h-screen">
            <ResumePage/>
        </section>
    }
}