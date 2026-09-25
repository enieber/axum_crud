use std::sync::{Arc, Mutex};
use crate::models::{Task};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        let seed = vec![
            Task { id: 1, title: "Learn Rust".to_string(), done: true },
            Task { id: 2, title: "Learn axum".to_string(), done: false },
            Task { id: 3, title: "Crate doc api".to_string(), done: false },
        ];

        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
}
