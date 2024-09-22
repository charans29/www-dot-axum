use crate::error_template::{AppError, ErrorTemplate};
use crate::components::home::HomePage;
use crate::components::about::AboutPage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/www-dot-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main class="bg-black h-screen w-screen">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=AboutPage/> 
                </Routes>
            </main>
        </Router>
    }
}

