use std::{io::Cursor, sync::Arc};

use hyper::server::conn::Http;
use log::info;
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{
        internal::pemfile::{certs, pkcs8_private_keys},
        NoClientAuth, ServerConfig,
    },
    TlsAcceptor,
};

use axum::{handler::get, response::IntoResponse, Json, Router};

async fn auth_handler() -> impl IntoResponse {
    info!("Got auth request");
    #[cfg(debug_assertions)]
    let server = "192.168.2.129";
    #[cfg(not(debug_assertions))]
    let server = "127.0.0.100";

    Json(serde_json::json!({
        "access_token": "aaaaaaaaaaaaaaaa",
        "ssid": "Test_SSID1234",
        "server": server
    }))
}

pub async fn io_server() -> anyhow::Result<()> {
    let app = Router::new().route("/authDevice", get(auth_handler));

    let mut tls_config = ServerConfig::new(NoClientAuth::new());
    let key = pkcs8_private_keys(&mut Cursor::new(include_bytes!("certs/key.pem")))
        .unwrap()
        .remove(0);
    let certs = certs(&mut Cursor::new(include_bytes!("certs/cert.pem"))).unwrap();
    tls_config.set_single_cert(certs, key).unwrap();
    tls_config.set_protocols(&[b"http/1.1".to_vec()]);

    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    #[cfg(debug_assertions)]
    let bind_addr = "0.0.0.0:443";
    #[cfg(not(debug_assertions))]
    let bind_addr = "127.0.0.1:443";

    let listener = TcpListener::bind(bind_addr).await?;

    loop {
        let (stream, _addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        let app = app.clone();

        tokio::spawn(async move {
            if let Ok(stream) = acceptor.accept(stream).await {
                let _ = Http::new().serve_connection(stream, app).await;
            }
        });
    }
}
