use std::sync::Arc;
use tokio::sync::Mutex;

type LogStore = Arc<Mutex<String>>;

#[derive(Clone)]
pub struct AppState {
    pub logs: LogStore,
    pub last_log: LogStore,
    pub openai_api_key: String,
    pub auth_key: String,
}

impl AppState {
    pub fn new(openai_api_key: String, auth_key: String) -> Self {
        Self {
            logs: Arc::new(Mutex::new(String::new())),
            last_log: Arc::new(Mutex::new(String::new())),
            openai_api_key,
            auth_key,
        }
    }
}
