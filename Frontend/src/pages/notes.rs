use yew::prelude::*;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use serde::{Deserialize, Serialize};
use gloo_console::log;
use serde_json::to_string;

use crate::pages::auth::use_auth_check;

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
struct Note {
    id: usize,
    title: String,
    content: String,
}

#[function_component(Notes)]
pub fn notes() -> Html {
    let (is_logged_in, _username) = use_auth_check();
    let notes = use_state(|| vec![]);
    let selected_note = use_state(|| None as Option<Note>);

    // Fetch notes on mount
    {
        let notes = notes.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                log!("📥 Fetching notes from backend...");
                // Retrieve stored access token
                let token = LocalStorage::get::<String>("access_token").unwrap_or_default();
                let resp = Request::get("http://localhost:8000/api/notes/")
                    .header("Authorization", &format!("Bearer {}", token))
                    .send()
                    .await;

                match resp {
                    Ok(resp) => {
                        let text = resp.text().await.unwrap_or_else(|_| "[]".to_string());
                        log!(format!("🪵 Raw response: {}", text));

                        match serde_json::from_str::<Vec<Note>>(&text) {
                            Ok(parsed) => notes.set(parsed),
                            Err(err) => log!(format!("❌ Failed to parse notes: {:?}", err)),
                        }
                    }
                    Err(err) => log!(format!("❌ Failed to fetch notes: {:?}", err)),
                }
            });
            || ()
        });
    }

  // Auto-save logic (runs every 5 seconds)
    {
        log!("🛠️ Entering auto-save setup block");
        let selected_note = selected_note.clone();
        log!(format!("🛠️ Entering auto‐save setup block; selected_note = {:?}", selected_note.as_ref())); 
        use_effect_with((), move |_| {
            log!("🚀 Auto-save effect initialized");
            let interval = Interval::new(5_000, move || {
                log!("⌛ Auto-save tick");
                if let Some(note) = (*selected_note).clone() {
                    log!(format!("💾 Auto-saving note: id={} title={}", note.id, note.title));
                    spawn_local({
                        let note = note.clone();
                        async move {
                            let token = LocalStorage::get::<String>("access_token").unwrap_or_default();
                            let endpoint = format!("http://localhost:8000/api/update/{}/", note.id);

                            // Try update existing note
                            let req = Request::put(&endpoint)
                                .header("Authorization", &format!("Bearer {}", token))
                                .json(&note)
                                .unwrap();

                            if let Err(err) = req.send().await {
                                // If PUT fails, fallback to POST create
                                log!(format!("❌ Update failed (will try create): {}", err));
                                let _ = Request::post("http://localhost:8000/api/notes/")
                                    .header("Authorization", &format!("Bearer {}", token))
                                    .json(&note)
                                    .unwrap()
                                    .send()
                                    .await;
                            }
                        }
                    });
                }
            });
            // cleanup interval on unmount
            move || {
                interval.cancel();
            }
        });
    }

    // Handlers for UI interactions...
    let on_note_click = {
        let selected_note = selected_note.clone();
        Callback::from(move |note: Note| selected_note.set(Some(note)))
    };

    let on_title_input = {
        let selected_note = selected_note.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            if let Some(mut note) = (*selected_note).clone() {
                note.title = input.value();
                selected_note.set(Some(note));
            }
        })
    };

    let on_content_input = {
        let selected_note = selected_note.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            if let Some(mut note) = (*selected_note).clone() {
                note.content = input.value();
                selected_note.set(Some(note));
            }
        })
    };

    // Render
    if !*is_logged_in {
        html! { <h1>{ "🚫 Please log in to view your notes." }</h1> }
    } else {
        html! {
            <div class="bg-gradient">
                <div class="bg-decoration-1"></div>
                <div class="bg-decoration-2"></div>

                <div class="sidebar">
                    <div class="sidebar-header">
                        <h2 class="sidebar-title">{ "Files" }</h2>
                    </div>
                    <div class="sidebar-content" style="overflow-y: auto; max-height: 80vh;">
                        {
                            for notes.iter().map(|note| {
                                let note = note.clone();
                                let note_for_callback = note.clone();
                                let click_handler = on_note_click.clone();
                                let onclick = Callback::from(move |_| {
                                    click_handler.emit(note_for_callback.clone());
                                });

                                html! {
                                    <a href="#" onclick={onclick}>
                                        { note.title.clone() }
                                    </a>
                                }
                            })
                        }
                    </div>
                </div>

                <div class="main-content">
                    <div class="typing-header">
                        <textarea
                            class="title-textarea"
                            placeholder="Enter Title..."
                            value={selected_note.as_ref().map_or("".to_string(), |n| n.title.clone())}
                            oninput={on_title_input}
                            autofocus=true
                        />
                    </div>
                    <div class="typing-area">
                        <textarea
                            class="typing-textarea"
                            placeholder="Start typing..."
                            value={selected_note.as_ref().map_or("".to_string(), |n| n.content.clone())}
                            oninput={on_content_input}
                        />
                    </div>
                    <div class="status-bar">
                        <div class="save-status">
                            <div class="save-indicator"></div>
                            <span>{ "Auto-saved" }</span>
                        </div>
                        <div class="actions">
                            <span class="word-count">
                                {
                                    selected_note
                                        .as_ref()
                                        .map_or(0, |n| n.content.split_whitespace().count())
                                }{ " words" }
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
