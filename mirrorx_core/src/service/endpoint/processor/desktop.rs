use crate::{
    component::{
        desktop::{Duplicator, Frame},
        video_decoder::DecodedFrame,
    },
    error::MirrorXError,
    ffi::ffmpeg::avutil::av_gettime_relative,
    service::endpoint::ffi::create_callback_fn,
    utility::runtime::TOKIO_RUNTIME,
};
use crossbeam::channel::{Receiver, Sender, TryRecvError, TrySendError};
use scopeguard::defer;
use std::{os::raw::c_void, time::Duration};
use tracing::{error, info, trace};

#[cfg(target_os = "windows")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: tokio::sync::broadcast::Sender<()>,
    mut exit_rx: tokio::sync::broadcast::Receiver<()>,
    capture_frame_tx: tokio::sync::mpsc::Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<(), MirrorXError> {
    use tokio::select;

    use crate::utility::runtime::TOKIO_RUNTIME;

    let mut duplicator = Duplicator::new(display_id)?;

    let expected_wait_time = Duration::from_secs_f32(1f32 / (fps as f32));

    TOKIO_RUNTIME.spawn(async move {
        defer! {
            let _ = exit_tx.send(());
            info!(?remote_device_id, "desktop capture process exit");
        }

        let mut interval = tokio::time::interval(expected_wait_time);
        let epoch = unsafe { av_gettime_relative() };

        loop {
            let process_time_start = std::time::Instant::now();

            select! {
                biased;

                _ = exit_rx.recv() => {
                    return;
                }

                _ = interval.tick() => {
                    match duplicator.capture() {
                        Ok(mut frame) => unsafe {
                            frame.capture_time = av_gettime_relative() - epoch;

                            trace!(
                                width=?frame.width,
                                height=?frame.height,
                                chrominance_len=?frame.chrominance_buffer.len(),
                                chrominance_stride=?frame.chrominance_stride,
                                luminance_len=?frame.luminance_buffer.len(),
                                luminance_stride=?frame.luminance_stride,
                                capture_time=?frame.capture_time,
                                "desktop capture frame",
                            );

                            if let Err(_) = capture_frame_tx.try_send(frame) {
                                info!("desktop frame channel disconnected");
                                return;
                            }
                        },
                        Err(err) => {
                            error!(?err, "capture desktop frame failed");
                            return;
                        }
                    };
                }
            }
        }
    });

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: tokio::sync::broadcast::Sender<()>,
    mut exit_rx: tokio::sync::broadcast::Receiver<()>,
    capture_frame_tx: tokio::sync::mpsc::Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<(), MirrorXError> {
    use crate::utility::runtime::TOKIO_RUNTIME;

    let mut duplicator = Duplicator::new(capture_frame_tx, display_id, fps)?;

    TOKIO_RUNTIME.spawn(async move {
        defer! {
            let _ = exit_tx.send(());
            info!(?remote_device_id, "desktop capture process exit");
        }

        if let Err(err) = duplicator.start() {
            error!(?err, "duplicator start failed");
            return;
        }

        let _ = exit_rx.recv().await;

        duplicator.stop();
    });

    Ok(())
}

pub fn start_desktop_render_process(
    remote_device_id: String,
    decoded_video_frame_rx: crossbeam::channel::Receiver<DecodedFrame>,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) {
    let update_callback_fn = unsafe { create_callback_fn(update_frame_callback_ptr) };

    let _ = std::thread::Builder::new()
        .name(format!("video_render_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let decoded_video_frame = match decoded_video_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => {
                        info!(?remote_device_id, "video decoded channel is closed");
                        break;
                    }
                };

                info!(
                    "begin render frame {}",
                    chrono::Utc::now().timestamp_millis()
                );
                #[cfg(target_os = "macos")]
                unsafe {
                    update_callback_fn(
                        texture_id,
                        video_texture_ptr as *mut c_void,
                        decoded_video_frame.0,
                    );
                }
                info!(
                    "begin render frame {}",
                    chrono::Utc::now().timestamp_millis()
                );

                #[cfg(target_os = "windows")]
                unsafe {
                    update_callback_fn(
                        video_texture_ptr as *mut c_void,
                        decoded_video_frame.0.as_ptr(),
                        1920,
                        1080,
                    );
                }
            }

            info!(?remote_device_id, "video render process exit");
        });
}
