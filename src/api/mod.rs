use anyhow::Result;
use http::{
    header::{HeaderName, HeaderValue},
    Response, StatusCode,
};
use std::net::SocketAddr;
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};

use crate::device::DeviceHandle;
use axum::{
    body::BoxBody,
    extract::{Extension, Path},
    handler::{get, post, Handler},
    response::IntoResponse,
    AddExtensionLayer, Json, Router,
};

mod dlna;
mod play_online;

fn static_header_override(
    name: &'static str,
    value: &'static str,
) -> SetResponseHeaderLayer<HeaderValue, Response<BoxBody>> {
    SetResponseHeaderLayer::<_, Response<BoxBody>>::overriding(
        HeaderName::from_static(name),
        HeaderValue::from_static(value),
    )
}

async fn handle_generic_cmd(
    Path(cid): Path<u32>,
    Json(req): Json<serde_json::Value>,
    Extension(dev): Extension<DeviceHandle>,
) -> impl IntoResponse {
    match dev.send_command(cid as usize, req).await {
        Ok(x) => Ok(Json(x)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn api_server(sender: DeviceHandle) -> Result<()> {
    let dlna_app = Router::new()
        .route("/dlna/desc", get(dlna::handle_desc))
        .route(
            "/dlna/scpd/:service",
            get(dlna::handle_scpd.layer(TraceLayer::new_for_http())),
        )
        .route(
            "/dlna/service/:service/action",
            post(dlna::handle_action.layer(TraceLayer::new_for_http())),
        )
        .layer(static_header_override(
            "server",
            "UPnP/1.0 DLNADOC/1.50 Tokio-DLNA-Device",
        ))
        .layer(static_header_override("ext", ""))
        .layer(static_header_override(
            "content-type",
            "text/xml; charset=\"utf-8\"",
        ));

    let app = Router::new()
        .route("/playOnline", post(play_online::handle))
        .route("/command/:id", post(handle_generic_cmd))
        .or(dlna_app)
        .layer(AddExtensionLayer::new(sender));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8233));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
