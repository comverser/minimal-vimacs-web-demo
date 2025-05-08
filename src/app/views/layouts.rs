use maud::{html, Markup, DOCTYPE};

pub fn base(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

                title { "VIMACS" };

                link rel="stylesheet" href="/assets/style.css";

                script defer src="/assets/script.js" {};

                script defer
                    src="https://unpkg.com/htmx.org@2.0.4"
                    integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+"
                    crossorigin="anonymous" {}
            }
            body {
                header class="brand-bar" {
                    span class="logo" { "Dall9" }
                    span { " × " }
                    span class="logo" { "Linearust" }
                }
                (content)
            }
        }
    }
}
