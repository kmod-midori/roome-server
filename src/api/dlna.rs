use axum::{
    async_trait,
    extract::{self, Extension, FromRequest, RequestParts},
    response::IntoResponse,
};
use bytes::Bytes;
use http::StatusCode;
use log::{debug, info};
use xmltree::Element;

use crate::device::DeviceHandle;

pub struct ExtractServiceAction(String);

#[async_trait]
impl<B> FromRequest<B> for ExtractServiceAction
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let soap_action = req
            .headers()
            .and_then(|headers| headers.get("soapaction"))
            .ok_or((StatusCode::BAD_REQUEST, "`SOAPAction` header is missing"))?;

        let soap_action = soap_action.to_str().or(Err((
            StatusCode::BAD_REQUEST,
            "`SOAPAction` header has invalid character",
        )))?;

        if let Some((_, service_action)) = soap_action[..soap_action.len() - 1].rsplit_once('#') {
            Ok(ExtractServiceAction(service_action.to_owned()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`SOAPAction` header is invalid"))
        }
    }
}

pub async fn handle_scpd(extract::Path(service_name): extract::Path<String>) -> impl IntoResponse {
    match service_name.as_str() {
        "AVTransport" => Ok(&include_bytes!("xml/AVTransport.xml")[..]),
        "ConnectionManager" => Ok(&include_bytes!("xml/ConnectionManager.xml")[..]),
        "RenderingControl" => Ok(&include_bytes!("xml/RenderingControl.xml")[..]),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn handle_desc() -> impl IntoResponse {
    &include_bytes!("xml/desc.xml")[..]
}

pub async fn handle_action(
    extract::Path(service_name): extract::Path<String>,
    ExtractServiceAction(service_action): ExtractServiceAction,
    body: Bytes,
    Extension(dev): Extension<DeviceHandle>,
) -> impl IntoResponse {
    let error_status = StatusCode::BAD_REQUEST;
    let error_msg = "SOAP parse failed";

    let root = Element::parse(body.as_ref()).or(Err((error_status, error_msg)))?;

    let action_body = root
        .get_child("Body")
        .and_then(|b| b.get_child(service_action.as_str()))
        .ok_or((error_status, error_msg))?;

    debug!("Service {}, action {}", service_name, service_action);

    match service_name.as_str() {
        "AVTransport" => match service_action.as_str() {
            "SetAVTransportURI" => {
                let url = action_body
                    .get_child("CurrentURI")
                    .and_then(|el| el.get_text())
                    .ok_or((StatusCode::BAD_REQUEST, "No URL specified"))?;

                info!("URL: {}", url);
                let _ = dev
                    .play_music_online(
                        url.parse()
                            .or(Err((StatusCode::BAD_REQUEST, "Invalid URL")))?,
                    )
                    .await;

                Ok(concat!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                    "\r\n",
                    r#"<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">"#,
                    r#"<s:Body><u:SetAVTransportURIResponse xmlns:u="urn:schemas-upnp-org:service:AVTransport:1"/></s:Body>"#,
                    "</s:Envelope>"
                ))
            }
            "Play" => Ok(concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                "\r\n",
                r#"<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">"#,
                r#"<s:Body><u:PlayResponse xmlns:u="urn:schemas-upnp-org:service:AVTransport:1"/></s:Body>"#,
                "</s:Envelope>"
            )),
            "Stop" => {
                let _ = dev.stop_music().await;

                Ok(concat!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                    "\r\n",
                    r#"<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">"#,
                    r#"<s:Body><u:StopResponse xmlns:u="urn:schemas-upnp-org:service:AVTransport:1"/></s:Body>"#,
                    "</s:Envelope>"
                ))
            }
            "GetPositionInfo" => Ok(""),
            "GetTransportInfo" => Ok(concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                "\r\n",
                r#"<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">"#,
                r#"<s:Body><u:GetTransportInfoResponse xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">"#,
                "<CurrentTransportState>PLAYING</CurrentTransportState>",
                "<CurrentTransportStatus>OK</CurrentTransportStatus>",
                "<CurrentSpeed>1</CurrentSpeed>",
                "</u:GetTransportInfoResponse></s:Body>",
                "</s:Envelope>"
            )),
            _ => Err((StatusCode::NOT_FOUND, "Action not found")),
        },
        "RenderingControl" => match service_action.as_str() {
            "GetVolume" => Ok(concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                "\r\n",
                r#"<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">"#,
                r#"<s:Body><u:StopResponse xmlns:u="urn:schemas-upnp-org:service:AVTransport:1"/></s:Body>"#,
                "</s:Envelope>"
            )),
            "SetVolume" => {
                let vol: u8 = action_body
                    .get_child("DesiredVolume")
                    .and_then(|el| el.get_text())
                    .and_then(|text| text.parse().ok())
                    .ok_or((StatusCode::BAD_REQUEST, "No/Invalid volume"))?;
                debug!("Set volume: {}", vol);
                let _ = dev.set_music_volume(if vol == 0 { 1 } else { vol }).await;

                Ok(concat!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>"#,
                    "\r\n",
                    r#"<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">"#,
                    r#"<s:Body><u:SetVolumeResponse xmlns:u="urn:schemas-upnp-org:service:RenderingControl:1"/></s:Body>"#,
                    "</s:Envelope>"
                ))
            }
            _ => Err((StatusCode::NOT_FOUND, "Action not found")),
        },
        _ => Err((StatusCode::NOT_FOUND, "Service not found")),
    }
}
