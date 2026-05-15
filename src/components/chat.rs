use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = match serde_json::from_str(&s) {
                    Ok(msg) => msg,
                    Err(e) => {
                        log::error!("failed to parse websocket message: {:?}", e);
                        return false;
                    }
                };
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        if let Some(data) = msg.data {
                            if let Ok(message_data) = serde_json::from_str(&data) {
                                self.messages.push(message_data);
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    if input.value().trim().is_empty() {
                        return false;
                    }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let active_users = self.users.len();
        let message_count = self.messages.len();

        html! {
            <div class="flex min-h-screen w-screen bg-slate-950 text-slate-100">
                <aside class="flex-none w-72 min-h-screen border-r border-slate-800 bg-slate-900">
                    <div class="border-b border-slate-800 p-5">
                        <p class="text-sm font-semibold uppercase tracking-wide text-cyan-300">{"YewChat Studio"}</p>
                        <h1 class="mt-1 text-2xl font-semibold">{"Idea room"}</h1>
                        <p class="mt-3 text-sm leading-6 text-slate-400">{"A compact room for messages, links, gifs, and tiny creative sparks."}</p>
                    </div>
                    <div class="grid grid-cols-2 gap-3 border-b border-slate-800 p-5">
                        <div class="rounded border border-slate-700 bg-slate-950 p-3">
                            <div class="text-2xl font-semibold text-cyan-300">{active_users}</div>
                            <div class="text-xs uppercase tracking-wide text-slate-400">{"Users"}</div>
                        </div>
                        <div class="rounded border border-slate-700 bg-slate-950 p-3">
                            <div class="text-2xl font-semibold text-emerald-300">{message_count}</div>
                            <div class="text-xs uppercase tracking-wide text-slate-400">{"Messages"}</div>
                        </div>
                    </div>
                    <div class="p-5">
                        <div class="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">{"Online now"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="mb-3 flex rounded border border-slate-800 bg-slate-950 p-3">
                                    <div>
                                        <img class="h-11 w-11 rounded-full border border-cyan-400/40" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow px-3">
                                        <div class="flex text-sm font-semibold text-slate-100">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-slate-400">
                                            {"Ready to chat"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </aside>
                <main class="grow min-h-screen flex flex-col">
                    <header class="flex items-center justify-between border-b border-slate-800 px-8 py-5">
                        <div>
                            <p class="text-sm uppercase tracking-wide text-cyan-300">{"Live channel"}</p>
                            <div class="text-2xl font-semibold">{"Creative Broadcast"}</div>
                        </div>
                        <div class="rounded border border-emerald-400/40 bg-emerald-400/10 px-4 py-2 text-sm text-emerald-200">{"JSON WebSocket active"}</div>
                    </header>
                    <div class="grid grow grid-cols-1 overflow-hidden lg:grid-cols-[1fr_18rem]">
                    <section class="overflow-auto border-r border-slate-800">
                    <div class="min-h-full px-8 py-6">
                        if self.messages.is_empty() {
                            <div class="flex min-h-full items-center justify-center">
                                <div class="max-w-md rounded border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
                                    <svg class="mx-auto h-16 w-16 text-cyan-300" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
                                        <path d="M10 14H38V31H21L13 38V31H10V14Z" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>
                                        <path d="M17 22H31M17 27H25" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>
                                    </svg>
                                    <h2 class="mt-4 text-xl font-semibold">{"No messages yet"}</h2>
                                    <p class="mt-2 text-sm leading-6 text-slate-400">{"Send a first note, a gif link, or a small idea to wake up the room."}</p>
                                </div>
                            </div>
                        } else {
                        {
                            self.messages.iter().map(|m| {
                                let avatar = self
                                    .users
                                    .iter()
                                    .find(|u| u.name == m.from)
                                    .map(|u| u.avatar.clone())
                                    .unwrap_or_else(|| format!("https://avatars.dicebear.com/api/adventurer-neutral/{}.svg", m.from));
                                html!{
                                    <div class="mb-5 flex w-full max-w-2xl items-end rounded border border-slate-800 bg-slate-900 p-4">
                                        <img class="mr-4 h-10 w-10 rounded-full border border-slate-700" src={avatar} alt="avatar"/>
                                        <div>
                                            <div class="text-sm font-semibold text-cyan-300">
                                                {m.from.clone()}
                                            </div>
                                            <div class="mt-2 text-sm leading-6 text-slate-200">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3 max-w-sm rounded border border-slate-700" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                        }
                    </div>
                    </section>
                    <aside class="hidden bg-slate-900 p-5 lg:block">
                        <div class="rounded border border-slate-800 bg-slate-950 p-5">
                            <p class="text-sm font-semibold uppercase tracking-wide text-amber-300">{"Spark ideas"}</p>
                            <div class="mt-4 space-y-3 text-sm leading-6 text-slate-300">
                                <p>{"Share one thing you learned today."}</p>
                                <p>{"Drop a gif link if words are too slow."}</p>
                                <p>{"Ask a question that makes the room smarter."}</p>
                            </div>
                        </div>
                        <div class="mt-4 rounded border border-slate-800 bg-slate-950 p-5">
                            <p class="text-sm font-semibold uppercase tracking-wide text-cyan-300">{"Why JSON"}</p>
                            <p class="mt-3 text-sm leading-6 text-slate-400">{"The client sends text frames, but the text contains structured JSON so the room can distinguish register, users, and message events."}</p>
                        </div>
                    </aside>
                    </div>
                    <footer class="flex border-t border-slate-800 px-6 py-4">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Write a message, idea, or .gif link" class="block h-12 w-full rounded border border-slate-700 bg-slate-900 px-4 text-slate-100 outline-none focus:border-cyan-400" name="message" required=true />
                        <button onclick={submit} class="ml-3 flex h-12 w-12 items-center justify-center rounded bg-cyan-400 text-slate-950">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </footer>
                </main>
            </div>
        }
    }
}
