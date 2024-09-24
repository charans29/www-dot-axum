use leptos::{component, view, IntoView};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="text-blue-600 text-center h-full bg-purple-950 py-14">
            <h1>"About ME"</h1>
            <p>"This is the about page."</p>
        </div>
    }
}