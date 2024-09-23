use leptos::{component, view, IntoView};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="text-blue-600 text-center mt-10 h-full bg-purple-950">
            <h1>"About Us"</h1>
            <p>"This is the about page."</p>
        </div>
    }
}