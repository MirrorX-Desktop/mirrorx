#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::api::*;
use flutter_rust_bridge::*;

// Section: imports

use crate::service::endpoint::message::DisplayInfo;
use crate::service::endpoint::message::GetDisplayInfoResponse;
use crate::service::endpoint::message::MouseEvent;
use crate::service::endpoint::message::MouseKey;
use crate::service::endpoint::message::StartMediaTransmissionResponse;

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_init(
    port_: i64,
    os_type: *mut wire_uint_8_list,
    os_version: *mut wire_uint_8_list,
    config_dir: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_os_type = os_type.wire2api();
            let api_os_version = os_version.wire2api();
            let api_config_dir = config_dir.wire2api();
            move |task_callback| init(api_os_type, api_os_version, api_config_dir)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_config_read_device_id(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "config_read_device_id",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| config_read_device_id(),
    )
}

#[no_mangle]
pub extern "C" fn wire_config_save_device_id(port_: i64, device_id: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "config_save_device_id",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_device_id = device_id.wire2api();
            move |task_callback| config_save_device_id(api_device_id)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_config_read_device_id_expiration(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "config_read_device_id_expiration",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| config_read_device_id_expiration(),
    )
}

#[no_mangle]
pub extern "C" fn wire_config_save_device_id_expiration(port_: i64, time_stamp: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "config_save_device_id_expiration",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_time_stamp = time_stamp.wire2api();
            move |task_callback| config_save_device_id_expiration(api_time_stamp)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_config_read_device_password(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "config_read_device_password",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| config_read_device_password(),
    )
}

#[no_mangle]
pub extern "C" fn wire_config_save_device_password(
    port_: i64,
    device_password: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "config_save_device_password",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_device_password = device_password.wire2api();
            move |task_callback| config_save_device_password(api_device_password)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_signaling_connect(port_: i64, remote_device_id: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "signaling_connect",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_remote_device_id = remote_device_id.wire2api();
            move |task_callback| signaling_connect(api_remote_device_id)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_signaling_connection_key_exchange(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "signaling_connection_key_exchange",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_remote_device_id = remote_device_id.wire2api();
            let api_password = password.wire2api();
            move |task_callback| {
                signaling_connection_key_exchange(api_remote_device_id, api_password)
            }
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_endpoint_get_display_info(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "endpoint_get_display_info",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_remote_device_id = remote_device_id.wire2api();
            move |task_callback| endpoint_get_display_info(api_remote_device_id)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_endpoint_start_media_transmission(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
    expect_fps: u8,
    expect_display_id: *mut wire_uint_8_list,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "endpoint_start_media_transmission",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_remote_device_id = remote_device_id.wire2api();
            let api_expect_fps = expect_fps.wire2api();
            let api_expect_display_id = expect_display_id.wire2api();
            let api_texture_id = texture_id.wire2api();
            let api_video_texture_ptr = video_texture_ptr.wire2api();
            let api_update_frame_callback_ptr = update_frame_callback_ptr.wire2api();
            move |task_callback| {
                endpoint_start_media_transmission(
                    api_remote_device_id,
                    api_expect_fps,
                    api_expect_display_id,
                    api_texture_id,
                    api_video_texture_ptr,
                    api_update_frame_callback_ptr,
                )
            }
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_endpoint_mouse_event(
    port_: i64,
    remote_device_id: *mut wire_uint_8_list,
    event: *mut wire_MouseEvent,
    x: f32,
    y: f32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "endpoint_mouse_event",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_remote_device_id = remote_device_id.wire2api();
            let api_event = event.wire2api();
            let api_x = x.wire2api();
            let api_y = y.wire2api();
            move |task_callback| endpoint_mouse_event(api_remote_device_id, api_event, api_x, api_y)
        },
    )
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MouseEvent {
    tag: i32,
    kind: *mut MouseEventKind,
}

#[repr(C)]
pub union MouseEventKind {
    Up: *mut MouseEvent_Up,
    Down: *mut MouseEvent_Down,
    Move: *mut MouseEvent_Move,
    ScrollWheel: *mut MouseEvent_ScrollWheel,
}

#[repr(C)]
#[derive(Clone)]
pub struct MouseEvent_Up {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct MouseEvent_Down {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct MouseEvent_Move {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct MouseEvent_ScrollWheel {
    field0: f32,
}

// Section: wrapper structs

// Section: static checks

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_mouse_event_0() -> *mut wire_MouseEvent {
    support::new_leak_box_ptr(wire_MouseEvent::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<MouseEvent> for *mut wire_MouseEvent {
    fn wire2api(self) -> MouseEvent {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MouseEvent>::wire2api(*wrap).into()
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<MouseEvent> for wire_MouseEvent {
    fn wire2api(self) -> MouseEvent {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Up);
                MouseEvent::Up(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Down);
                MouseEvent::Down(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Move);
                MouseEvent::Move(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.ScrollWheel);
                MouseEvent::ScrollWheel(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<MouseKey> for i32 {
    fn wire2api(self) -> MouseKey {
        match self {
            0 => MouseKey::None,
            1 => MouseKey::Left,
            2 => MouseKey::Right,
            3 => MouseKey::Wheel,
            _ => unreachable!("Invalid variant for MouseKey: {}", self),
        }
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_MouseEvent {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_Up() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        Up: support::new_leak_box_ptr(MouseEvent_Up {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_Down() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        Down: support::new_leak_box_ptr(MouseEvent_Down {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_Move() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        Move: support::new_leak_box_ptr(MouseEvent_Move {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MouseEvent_ScrollWheel() -> *mut MouseEventKind {
    support::new_leak_box_ptr(MouseEventKind {
        ScrollWheel: support::new_leak_box_ptr(MouseEvent_ScrollWheel {
            field0: Default::default(),
        }),
    })
}

// Section: impl IntoDart

impl support::IntoDart for DisplayInfo {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.id.into_dart(),
            self.name.into_dart(),
            self.refresh_rate.into_dart(),
            self.width.into_dart(),
            self.height.into_dart(),
            self.is_primary.into_dart(),
            self.screen_shot.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for DisplayInfo {}

impl support::IntoDart for GetDisplayInfoResponse {
    fn into_dart(self) -> support::DartCObject {
        vec![self.displays.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for GetDisplayInfoResponse {}

impl support::IntoDart for StartMediaTransmissionResponse {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.os_name.into_dart(),
            self.os_version.into_dart(),
            self.screen_width.into_dart(),
            self.screen_height.into_dart(),
            self.video_type.into_dart(),
            self.audio_type.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for StartMediaTransmissionResponse {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
