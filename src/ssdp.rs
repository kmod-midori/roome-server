use std::{net::IpAddr, time::Duration};

use log::info;
use nix::{net::if_::InterfaceFlags, sys::socket::SockAddr};
use tokio_ssdp::{Device, Server};

const DEVICE_UUID: &str = "a1ab85e9-e299-4005-a427-f7e49cb1e119";

pub async fn ssdp_server() -> anyhow::Result<()> {
    let addr = loop {
        let addr = tokio::task::spawn_blocking(|| {
            let addrs = nix::ifaddrs::getifaddrs().ok()?;

            for ifaddr in addrs {
                if !ifaddr.flags.contains(InterfaceFlags::IFF_UP)
                    || ifaddr.flags.contains(InterfaceFlags::IFF_LOOPBACK)
                {
                    // Ignore DOWN or LOOPBACK
                    continue;
                }

                if let Some(SockAddr::Inet(addr)) = ifaddr.address {
                    if let IpAddr::V4(addr) = addr.to_std().ip() {
                        return Some(addr);
                    }
                }
            }
            None
        }).await?;

        if let Some(addr) = addr {
            break addr;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    };

    info!("Public interface address: {}", addr);

    let location = format!("http://{}:8233/dlna/desc", addr);

    // Based on https://github.com/xfangfang/Macast/blob/main/macast/plugin.py
    Server::new([
        // Device
        Device::new(DEVICE_UUID, "upnp:rootdevice", &location),
        Device::new(DEVICE_UUID, "", &location),
        // MediaRenderer
        Device::new(
            DEVICE_UUID,
            "urn:schemas-upnp-org:device:MediaRenderer:1",
            &location,
        ),
    ])
    .serve()?
    .await?;

    Ok(())
}
