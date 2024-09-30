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
            <main class="h-screen w-screen overflow-y-scroll"
                style="background-image: linear-gradient(180deg, rgba(5,0,5,0.65) 30%, rgba(15,1,4,1) 70%);"
            >
                <div class="flex justify-center z-10 inset-1 absolute h-[50px] items-center">
                    <nav class="border w-1/6 border-yellow-600 rounded-2xl opacity-0.1 flex justify-center space-x-5 mt-5 py-1 bg-slate-800">
                        <a href="/" class="font-mono text-red-700">"Home"</a>
                        <a href="/about" class="font-mono text-red-700">"About"</a>
                    </nav>
                </div>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=AboutPage/> 
                </Routes>
            </main>
        </Router>
    }
}

