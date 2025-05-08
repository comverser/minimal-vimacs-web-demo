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
            h2 { "Camera Preview" }
            video id="webcam" autoplay playsinline {}

            div .button-group {
                button id="startBtn" { "Start" }
                button id="stopBtn" { "Stop" }
            }
        }
    }
}

pub fn log_pane() -> Markup {
    html! {
        section.log-pane {
            h2 { "AI-Generated Procedure Log" }
            ul #log hx-get="/api/log" hx-trigger="every 1s" hx-swap="afterbegin" {}
        }
    }
}
