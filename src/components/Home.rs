use leptos::*;
use crate::components::resume::ResumePage;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-between h-screen font-thin w-screen py-5"
            style="background-image: linear-gradient(180deg, rgba(65, 247, 193, 0) 30%, rgba(13, 251, 203, 0.1) 100%);"
        >

            <div class="w-full flex flex-row justify-center p-10"> 
                <div class="h-80 w-64 overflow-hidden relative"> 
                    <img class="zoom-effect scale-x-[1.8] scale-y-[1.3] img-effect gradient-overlay" style="width: 150%; height: 150%; margin: -1%;" src="me.png" alt="Background Image"/>
                    <div class="gradient-overlay absolute inset-0 pointer-events-none"></div>
                </div> 
            </div>

        </div>

        <section id="resume" class="h-screen w-screen">
            <ResumePage/>
        </section>
    }
}