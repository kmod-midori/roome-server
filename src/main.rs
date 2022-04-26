//! `io.myroome.com`: 443 (HTTPS)
//! API/DLNA: 8233 (HTTP)
//! Roome Control: 9898 (TCP)
//! SSDP: 1900 (UDP)

use anyhow::Result;
use device::AlarmCommandReceiver;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

use crate::device::AlarmCommandBundle;

mod api;
mod device;
mod device_socket;
mod io_server;
mod ssdp;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<AlarmCommandBundle>(1);

    let sender = device::DeviceHandle::new(cmd_tx);
    let sender_c = sender.clone();

    tokio::spawn(async { io_server::io_server().await });
    tokio::spawn(async { api::api_server(sender_c).await });
    tokio::spawn(async { ssdp::ssdp_server().await });

    let listener = TcpListener::bind("0.0.0.0:9898").await?;
    info!("Listening on 0.0.0.0:9898");

    loop {
        let (socket, addr) = listener.accept().await?;

        info!("Got conn from {}", addr);

        let handle_res = handle_conn(socket, &mut cmd_rx, sender.clone()).await;

        if let Err(e) = handle_res {
            error!("Handle error: {}", e);
        }
    }
}

async fn handle_conn(
    socket: TcpStream,
    cmd_rx: &mut AlarmCommandReceiver,
    sender: device::DeviceHandle,
) -> Result<()> {
    device_socket::device_socket_task(socket, cmd_rx, sender).await?;

    Ok(())
}
