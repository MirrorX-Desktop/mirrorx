use crate::{
    api::endpoint::{
        client::ClientSendStream,
        message::{
            EndPointDownloadFileReply, EndPointDownloadFileRequest, EndPointFileTransferError,
            EndPointMessage,
        },
    },
    component::fs::transfer::send_file_to_remote,
    core_error,
    error::CoreResult,
};
use std::time::Duration;

pub async fn handle_download_file_request(
    client_send_stream: ClientSendStream,
    req: EndPointDownloadFileRequest,
) -> CoreResult<EndPointDownloadFileReply> {
    if !req.path.is_file() {
        return Err(core_error!("file not exists"));
    }

    let id = req.id.clone();
    let meta = req.path.metadata()?;
    let size = meta.len();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        if let Err(err) =
            send_file_to_remote(id.clone(), client_send_stream.clone(), &req.path).await
        {
            tracing::error!(?err, "read file block failed");
            let _ = client_send_stream
                .send(&EndPointMessage::FileTransferError(
                    EndPointFileTransferError { id: id.clone() },
                ))
                .await;
        }
    });

    Ok(EndPointDownloadFileReply { size })
}
