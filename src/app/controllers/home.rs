use crate::app::views::{components, layouts};
use chrono::Local;
use maud::{html, Markup};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

pub async fn index() -> Markup {
    layouts::base(components::main_container())
}

pub async fn log() -> Markup {
    const MESSAGES: [&str; 4] = ["Performing CPR", "Applying BVM", "Idle", "Starting IV"];

    let idx = NEXT.fetch_add(1, Ordering::Relaxed) % MESSAGES.len();
    let msg = MESSAGES[idx];

    let now = Local::now().format("%T").to_string();

    html! {
        li { "[" (now) "] LLM: " (msg) }
    }
}
