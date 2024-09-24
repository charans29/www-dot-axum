use leptos::{component, create_signal, view, IntoView, SignalUpdate};
use crate::components::about::AboutPage;

#[component]
pub fn HomePage() -> impl IntoView {
    
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="text-blue-600 text-center h-full bg-violet-950 py-14">
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
        <section id="about" class="h-screen">
            <AboutPage/>
        </section>
    }
}