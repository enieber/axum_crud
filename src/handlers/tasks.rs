use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::models::{CreateTask, Task, UpdateTask};
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) ->  Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    Json(tasks.clone())
}

pub async fn create(State(state): State<AppState>, Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let mut tasks = state.tasks.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let task = Task {
        id: *next_id,
        title: payload.title,
        done: false,
    };

    *next_id += 1;
    tasks.push(task.clone());
    (StatusCode::CREATED, Json(task.clone()))
}

pub async fn get_one(State(state): State<AppState>, Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();

    tasks
        .iter()
        .find(|t| t.id == id)
        .map(|t| Json(t.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}


pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
       let mut tasks = state.tasks.lock().unwrap();
       let task = tasks
           .iter_mut()
           .find(|t| t.id == id)
           .ok_or(StatusCode::NOT_FOUND)?;

       if let Some(title) = payload.title {
           task.title = title
       }

       if let Some(done) = payload.done {
           task.done = done
       }

       Ok(Json(task.clone()))
}


pub async fn remove(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let mut tasks = state.tasks.lock().unwrap();
    let before = tasks.len();

    tasks.retain(|t| t.id != id);

    if tasks.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
