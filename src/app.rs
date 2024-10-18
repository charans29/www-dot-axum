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
            .into_view()
        }>
            <main class="h-screen w-screen overflow-y-scroll css-selector"
            // style="
            //     background-image: linear-gradient(135deg,#1e1e20,#0e0e11 82%);"
            >
                <Routes>
                    <Route path="/" view= || HomePage/>
                    <Route path="/resume" view=ResumePage/> 
                </Routes>      
            </main>
        </Router>
    }
}

