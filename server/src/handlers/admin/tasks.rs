use std::{str::FromStr, sync::Arc};

use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    models::AppState,
    utils::{AppError, AppResult},
};

pub async fn get_running_tasks(State(state): State<Arc<AppState>>) -> AppResult<Json<Vec<String>>> {
    let running_tasks = {
        let tasks = state.running_entitlements_tasks.read().await;
        tasks.keys().map(|v| v.to_string()).collect()
    };

    Ok(Json(running_tasks))
}

pub async fn stop_running_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> AppResult<()> {
    let uuid =
        Uuid::from_str(&task_id).map_err(|_| anyhow!("provided task_id not not an uuidv4"))?;
    let task = {
        let mut tasks = state.running_entitlements_tasks.write().await;
        tasks.remove(&uuid)
    };

    match task {
        Some(handle) => {
            handle.abort();
            log::info!("successfully aborted task {uuid}");
            Ok(())
        }
        None => Err(AppError::from(anyhow!("unknown task with id {uuid}"))),
    }
}
