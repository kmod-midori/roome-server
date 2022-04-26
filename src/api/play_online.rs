use axum::{extract::Extension, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use super::DeviceHandle;

#[derive(Deserialize)]
pub struct PlayOnlineReq {
    url: String,
}

#[derive(Serialize)]
struct PlayOnlineCmd {
    url: String,
    time: String,
    keep: u32,
}

pub async fn handle(
    Json(req): Json<PlayOnlineReq>,
    Extension(dev): Extension<DeviceHandle>,
) -> impl IntoResponse {
    let _ = dev.play_music_online(req.url.parse().unwrap()).await;

    Ok::<_, Infallible>(())
}
