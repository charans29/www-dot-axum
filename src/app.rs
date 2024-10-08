use crate::error_template::{AppError, ErrorTemplate};
use crate::components::home::HomePage;
use crate::components::resume::ResumePage;
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
        }>
            <main class="h-screen w-screen overflow-y-scroll css-selector">
                <Routes>
                <Route path="/" view=|cx| <HomePage cx={cx} /> />
                    <Route path="/resume" view=|cx| <ResumePage cx/> /> 
                </Routes>      
            </main>
        </Router>
    }
}

