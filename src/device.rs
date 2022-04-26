use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};
use url::Url;

/// Commands sent to clock
#[derive(Debug, Clone, Serialize)]
struct AlarmCommand {
    k: [usize; 1],
    v: [serde_json::Value; 1],
}

/// Messages from clock
#[derive(Debug, Clone, Deserialize)]
pub struct AlarmMessage {
    pub k: Vec<usize>,
    pub v: Vec<serde_json::Value>,
    pub c: Option<u64>,
    pub t: Option<String>,
}

#[derive(Debug)]
pub struct AlarmCommandBundle {
    pub id: usize,
    pub resp_tx: oneshot::Sender<Option<i64>>,
    pub buf: Vec<u8>,
}
pub type AlarmCommandSender = Sender<AlarmCommandBundle>;
pub type AlarmCommandReceiver = Receiver<AlarmCommandBundle>;

#[derive(Clone)]
pub struct DeviceHandle {
    sender: AlarmCommandSender,
}

impl DeviceHandle {
    pub fn new(sender: AlarmCommandSender) -> Self {
        Self { sender }
    }

    pub async fn send_command(
        &self,
        id: usize,
        command: impl Serialize,
    ) -> anyhow::Result<Option<i64>> {
        let inner_val = serde_json::to_value(command)?;

        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = AlarmCommand {
            k: [id],
            v: [inner_val],
        };
        let buf = serde_json::to_vec(&cmd)?;

        self.sender
            .send(AlarmCommandBundle { id, resp_tx, buf })
            .await?;
        Ok(resp_rx.await?)
    }

    pub async fn play_music_online(&self, mut url: Url) -> anyhow::Result<()> {
        if url
            .host_str()
            .map(|host| host.ends_with(".bilivideo.com"))
            .unwrap_or(false)
        {
            if let Some(query) = url.query() {
                // Switch to "audio only" if possible (for live streams)
                let new_query = query.replace("ptype=0", "ptype=1");
                url.set_query(Some(&new_query))
            }
        }

        if url.scheme() == "https" {
            url.set_scheme("http").unwrap();
        }

        let cmd = commands::PlayOnlineCmd {
            url: url.into(),
            time: "".into(),
            keep: 0,
        };
        self.send_command(202, &cmd).await?;

        Ok(())
    }

    pub async fn stop_music(&self) -> anyhow::Result<()> {
        let cmd = commands::PlayOnlineCmd {
            url: "AAAAAAAAAAAAAAAAAAAA".into(),
            time: "".into(),
            keep: 0,
        };
        self.send_command(202, &cmd).await?;

        Ok(())
    }

    pub async fn set_music_volume(&self, volume: u8) -> anyhow::Result<()> {
        self.send_command(704, volume).await?;

        Ok(())
    }
}

mod commands {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct PlayOnlineCmd {
        pub url: String,
        pub time: String,
        pub keep: u32,
    }
}
