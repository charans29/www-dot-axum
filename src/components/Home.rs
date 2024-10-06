use leptos::{component, view, IntoView};
use crate::components::resume::ResumePage;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="flex flex-col justify-between h-screen font-thin w-screen py-5">
            <div class="text-white/90 flex justify-between px-10 items-center">
                <h1 class="bg-red-700 text-black p-3 rounded-sm">"Welcome to Leptos!"</h1>
                <div class="flex flex-row justify-between w-1/4">
                    <h1 class="cursor-pointer">"LinkedIn"</h1>
                    <h1 class="cursor-pointer">"GitHub"</h1>
                    <h1 class="cursor-pointer">"Twitter"</h1>
                    <h1 class="cursor-pointer">"Docs"</h1>
                </div>
                </div>
            <div class="text-white/90 flex justify-between px-10 items-end">
                <h1>"Welcome to Leptos!"</h1>
                <div class="flex flex-col justify-between">
                    <h1 class="cursor-pointer text-sm mb-1">"EDUCATION:"</h1>
                    <div class="flex space-x-1 ">
                        <h1 class="cursor-pointer border-[0.25px] border-gray-500 p-1 text-sm">"MS in CS"</h1>
                        <h1 class="cursor-pointer border-[0.25px] border-gray-500 p-1 text-sm">"CSU CHICO'24"</h1>
                        <h1 class="cursor-pointer border-[0.25px] border-gray-500 p-1 text-sm">"USA"</h1>
                    </div>
                </div>
                <h1>"Welcome to Leptos!"</h1>
            </div>
        </div>
        <section id="resume" class="h-screen w-screen">
            <ResumePage/>
        </section>
    }
}