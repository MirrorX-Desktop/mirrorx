use tracing::info;

use super::{
    endpoint::EndPoint,
    message::{MediaFrame, StartMediaTransmissionResponse},
};
use crate::{
    error::MirrorXError,
    socket::endpoint::{endpoint::ENDPOINTS, message::StartMediaTransmissionRequest},
};

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> Result<StartMediaTransmissionResponse, MirrorXError> {
    info!("receive handle start media transmission");
    endpoint.begin_screen_capture()?;
    info!("begin screen capture end");

    let reply = StartMediaTransmissionResponse {
        os_name: crate::constants::OS_NAME
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("Unknown")),
        os_version: crate::constants::OS_VERSION
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("Unknown")),
        video_type: String::from("todo"),
        audio_type: String::from("todo"),
    };

    Ok(reply)
}

pub async fn handle_media_transmission(
    remove_device_id: String,
    media_transmission: MediaFrame,
) -> Result<(), MirrorXError> {
    // info!(
    //     "receive media transmission, length: {}",
    //     media_transmission.data.len()
    // );

    if let Some(endpoint) = ENDPOINTS.get(&remove_device_id) {
        endpoint.transfer_desktop_video_frame(media_transmission.data);
    };

    info!("receive media");

    Ok(())
}
