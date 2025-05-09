use crate::app::{
    state::AppState,
    views::{components, layouts},
};
use axum::{body::Bytes, extract::State, http::StatusCode};
use base64::{engine::general_purpose, Engine};
use chrono::Local;
use maud::{html, Markup};
use reqwest::Client;
use serde_json::{json, Value};

// TODO: add backned logging
// TODO: add error handling
// TODO: use app state type
// TODO: use hashmap in app state
// TODO: refactor the error handling

pub async fn get_index() -> Markup {
    layouts::base(components::main_container())
}

pub async fn post_frame(
    State(app_state): State<AppState>,
    bytes: Bytes,
) -> Result<StatusCode, (StatusCode, String)> {
    let base64_image = general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:image/webp;base64,{base64_image}");

    let prompt_text = r#"<System>
Task: return **one** label (exact text, no extras) for each image:

cpr intubation ventilation aed iv mechcpr nasalcannula facemask o2_supply
bvm igel tourniquet cspine_collar aed_device iv_catheter none

Rules  
1. Action visible → use its procedure label.  
2. Only idle gear → use its device label.  
3. Unclear frame → none.  
4. Output label only.

Hints:  
• CPR = arms locked over chest • Intubation = laryngoscope/tube in mouth  
• Ventilation = BVM on face   • AED = pads on chest or device open  
• IV = tourniquet + cannula    • mechcpr = piston frame on chest  
• I-Gel = supraglottic airway  • Collar = neck brace
</System>
"#;

    let payload = json!({
        "model": "gpt-4o-mini",
        "input": [{
            "role": "user",
            "content": [
                { "type": "input_text",  "text": prompt_text },
                { "type": "input_image", "image_url": data_url, "detail": "high" }
            ]
        }]
    });

    // Test mode: Bypass API call when in test mode.
    if app_state.openai_api_key == "TEST" {
        let now = Local::now().format("%T").to_string();
        let caption = format!("[TEST] {}", now);
        let mut log = app_state.logs.lock().await;
        *log = caption;
        return Ok(StatusCode::OK);
    }

    let openai_api_key = app_state.openai_api_key;

    let response: Value = Client::new()
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(openai_api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .error_for_status()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let caption = response["output"][0]["content"][0]["text"]
        .as_str()
        .unwrap_or("[no caption]")
        .to_owned();

    let mut log = app_state.logs.lock().await;
    *log = caption;

    Ok(StatusCode::OK)
}

pub async fn get_log(State(app_state): State<AppState>) -> Markup {
    let current = app_state.logs.lock().await.clone();

    {
        let mut last = app_state.last_log.lock().await;
        if *last == current || "none" == current {
            // unchanged  →  tell client “nothing new”
            return html! {};
        }
        *last = current.clone();
    }

    let now = Local::now().format("%T").to_string();

    html! {
        li { "[" (now) "]  " (current) }
    }
}
