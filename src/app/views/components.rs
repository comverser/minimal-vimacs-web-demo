use maud::{html, Markup};

pub fn main_container() -> Markup {
    html! {
        main class="container" {
            (video_pane())
            (log_pane())
        }
    }
}

pub fn video_pane() -> Markup {
    html! {
        section .video-pane {
            h2 { "Live Video" }
            video id="webcam" autoplay playsinline {}
            button id="startBtn" { "Start Demo" }
        }
    }
}

pub fn log_pane() -> Markup {
    html! {
        section.log-pane {
            h2 { "LLM Log" }
            ul #log hx-get="/api/log" hx-trigger="every 3s" hx-swap="beforeend" {}

        }
    }
}
