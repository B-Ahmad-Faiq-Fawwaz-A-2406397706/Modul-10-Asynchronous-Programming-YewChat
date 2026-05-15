use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="flex min-h-screen w-screen bg-slate-950 text-slate-100">
            <div class="mx-auto grid w-full max-w-6xl grid-cols-1 items-center gap-8 px-6 py-10 lg:grid-cols-2">
                <section class="space-y-6">
                    <div class="inline-flex items-center gap-3 rounded border border-cyan-400/40 bg-slate-900 px-4 py-2 text-sm font-semibold uppercase tracking-wide text-cyan-200">
                        <span class="flex h-8 w-8 items-center justify-center rounded bg-cyan-400 text-slate-950">{"YC"}</span>
                        <span>{"Creative WebSocket Room"}</span>
                    </div>
                    <div class="space-y-4">
                        <h1 class="max-w-xl text-5xl font-semibold leading-tight">{"A tiny chat room for fast ideas."}</h1>
                        <p class="max-w-lg text-lg leading-8 text-slate-300">
                            {"This client keeps the tutorial flow, but adds a stronger visual identity, a clearer entry moment, and a little more personality before users enter the room."}
                        </p>
                    </div>
                    <div class="grid max-w-xl grid-cols-3 gap-3 text-sm text-slate-300">
                        <div class="rounded border border-slate-800 bg-slate-900 p-4">
                            <div class="text-2xl font-semibold text-cyan-300">{"01"}</div>
                            <div class="mt-2">{"Pick a name"}</div>
                        </div>
                        <div class="rounded border border-slate-800 bg-slate-900 p-4">
                            <div class="text-2xl font-semibold text-emerald-300">{"02"}</div>
                            <div class="mt-2">{"Join live"}</div>
                        </div>
                        <div class="rounded border border-slate-800 bg-slate-900 p-4">
                            <div class="text-2xl font-semibold text-amber-300">{"03"}</div>
                            <div class="mt-2">{"Share sparks"}</div>
                        </div>
                    </div>
                </section>

                <section class="rounded border border-slate-800 bg-slate-900 p-6 shadow-2xl">
                    <div class="mb-6 flex items-center justify-between">
                        <div>
                            <p class="text-sm uppercase tracking-wide text-cyan-300">{"Enter the room"}</p>
                            <h2 class="text-2xl font-semibold">{"Start with a nickname"}</h2>
                        </div>
                        <svg class="h-14 w-14 text-cyan-300" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <rect x="7" y="10" width="34" height="24" rx="4" stroke="currentColor" stroke-width="3"/>
                            <path d="M16 38L22 34H34" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>
                            <circle cx="17" cy="22" r="2" fill="currentColor"/>
                            <circle cx="24" cy="22" r="2" fill="currentColor"/>
                            <circle cx="31" cy="22" r="2" fill="currentColor"/>
                        </svg>
                    </div>
                    <form class="flex flex-col gap-4">
                        <input {oninput} class="h-12 rounded border border-slate-700 bg-slate-950 px-4 text-slate-100 outline-none focus:border-cyan-400" placeholder="Username" />
                        <Link<Route> to={Route::Chat}>
                            <button {onclick} disabled={username.len()<1} class="h-12 w-full rounded bg-cyan-400 px-5 font-bold text-slate-950 disabled:cursor-not-allowed disabled:bg-slate-700 disabled:text-slate-400">
                                {"Go Chatting"}
                            </button>
                        </Link<Route>>
                    </form>
                    <p class="mt-5 text-sm leading-6 text-slate-400">
                        {"The username is sent as a register message, so the server can announce who is currently inside the room."}
                    </p>
                </section>
            </div>
        </div>
    }
}
