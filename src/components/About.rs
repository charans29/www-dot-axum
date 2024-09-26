use leptos::{component, view, IntoView};
use crate::components::docs::DocsPage;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="text-blue-600 text-center h-full py-14">
            <h1>"About ME"</h1>
            <p>"This is the about page."</p>
        </div>
        <section id="about" class="h-screen">
            <DocsPage/>
        </section>
    }
}