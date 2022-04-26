use anyhow::{bail, Result};
use bytes::BytesMut;
use log::debug;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::oneshot};

use crate::device::{AlarmCommandReceiver, AlarmMessage, DeviceHandle};

pub async fn device_socket_task(
    mut stream: TcpStream,
    cmd_rx: &mut AlarmCommandReceiver,
    dev: DeviceHandle,
) -> Result<()> {
    let mut read_buf = BytesMut::with_capacity(4096);
    let mut write_buf = vec![];
    let mut pending_responses = vec![];

    loop {
        tokio::select! {
            res = stream.readable() => {
                res?;

                match stream.try_read_buf(&mut read_buf) {
                    Ok(0) => bail!("Stream EOF"),
                    Ok(_) => {
                        process_messages(&mut read_buf, &dev, &mut pending_responses)?;
                    },
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

            res = stream.writable(), if !write_buf.is_empty() => {
                res?;

                stream.write_all(&write_buf).await?;
                write_buf = vec![];
            }

            cmd = cmd_rx.recv(), if write_buf.is_empty() => {
                match cmd {
                    Some(cmd) => {
                        write_buf = cmd.buf;
                        pending_responses.push((cmd.id, cmd.resp_tx));
                    },
                    None => bail!("All device handle dropped"),
                }
            }
        }
    }
}

fn process_messages(
    buf: &mut BytesMut,
    dev: &DeviceHandle,
    pending_responses: &mut Vec<(usize, oneshot::Sender<Option<i64>>)>,
) -> Result<()> {
    const BRACE_IN: u8 = b'{';
    const BRACE_OUT: u8 = b'}';

    let mut message_lengths = vec![];
    let mut current_len = 1;
    let mut brace_open = 1;
    let mut brace_close = 0;

    for ch in buf.iter().skip(1) {
        match *ch {
            BRACE_IN => brace_open += 1,
            BRACE_OUT => brace_close += 1,
            _ => (),
        }
        current_len += 1;

        if brace_open == brace_close {
            message_lengths.push(current_len);
            current_len = 0;
            brace_open = 0;
            brace_close = 0;
        }
    }

    for sp in message_lengths {
        let buf_msg = buf.split_to(sp);
        let str_msg = String::from_utf8_lossy(&buf_msg);
        let msg: AlarmMessage = serde_json::from_str(&str_msg)?;

        match msg.k[0] {
            1 => {
                let dev = dev.clone();

                tokio::spawn(async move {
                    if let Ok(res) = nippy::get_unix_ntp_time().await {
                        let _ = dev.send_command(10, res).await;
                    }
                });
            }
            2 => {
                debug!("Heartbeat: {:?}", msg);
            }
            n => {
                debug!("Device message {:?}", msg);
                if let Some(pos) = pending_responses.iter().position(|(id, _)| *id == n) {
                    let (_, resp_tx) = pending_responses.remove(pos);
                    let v = msg.v[0].as_i64();
                    let _ = resp_tx.send(v);
                }
            }
        }
    }

    Ok(())
}
