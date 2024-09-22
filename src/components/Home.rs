use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn HomePage() -> impl IntoView {
    
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="text-blue-600 text-center mt-10">
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}