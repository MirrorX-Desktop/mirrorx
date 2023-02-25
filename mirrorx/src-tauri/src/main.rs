#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod utility;
mod window;

use std::sync::Arc;

use mirrorx_core::{
    api::{config::LocalStorage, endpoint::client::EndPointClient, signaling::SignalingClient},
    error::CoreResult,
};
use moka::sync::{Cache, CacheBuilder};
#[cfg(target_os = "macos")]
use tauri::Icon;

use tauri::{App, Manager, SystemTray, SystemTrayEvent, WindowEvent};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[cfg(target_os = "macos")]
static TRAY_ICON_MACOS: &[u8] = include_bytes!("../assets/icons/tray-macOS.png");

#[tokio::main]
#[tracing::instrument]
async fn main() {
    let app = match init_app() {
        Ok(app) => app,
        Err(err) => {
            let message = format!("Init app failed, please relaunch app!\nError: {}", err);

            let _ = native_dialog::MessageDialog::new()
                .set_title("MirrorX Runtime Error")
                .set_text(&message)
                .set_type(native_dialog::MessageType::Error)
                .show_alert();

            return;
        }
    };

    app.run(|app_handle, event| match event {
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == "main" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if let Some(window) = app_handle.get_window(&label) {
                        let _ = window.hide();
                        api.prevent_close();
                    }
                }
            }
        }
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn init_app() -> anyhow::Result<App> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let app = init_tauri()?;
    init_component(&app)?;

    Ok(app)
}

fn init_tauri() -> anyhow::Result<App> {
    let tray = SystemTray::new();
    #[cfg(target_os = "macos")]
    let tray = tray
        .with_icon(Icon::Raw(TRAY_ICON_MACOS.to_vec()))
        .with_icon_as_template(true);

    let app = tauri::Builder::default()
        .manage(mirrorx_core::component::client::portal::Client::default())
        .manage(mirrorx_core::component::lan::LANProvider::new())
        .manage(Arc::new(
            CacheBuilder::<String, Arc<EndPointClient>, _>::new(64).build(),
        ))
        .system_tray(tray)
        .enable_macos_default_menu(false)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::DoubleClick { .. } = event {
                app.windows().values().for_each(|window| {
                    let _ = window.show();
                    let _ = window.unminimize();
                })
            }
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => std::process::exit(0),
                    "show" => app.windows().values().for_each(|window| {
                        let _ = window.show();
                    }),
                    "hide" => app.windows().values().for_each(|window| {
                        let _ = window.hide();
                    }),
                    "about" => {
                        let _ = app.emit_all("/dialog/about", ());
                    }
                    _ => {}
                }
            }
        })
        .on_menu_event(|event| {
            if event.menu_item_id() == "about" {
                let _ = event.window().emit("/dialog/about", ());
            }

            if event.menu_item_id() == "quit" {
                std::process::exit(0)
            }
        })
        .setup(|app| {
            app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));
            let app_name = app.package_info().name.clone();

            let handle = app.handle();
            std::thread::spawn(move || {
                let builder = tauri::WindowBuilder::new(
                    &handle,
                    "main",
                    tauri::WindowUrl::App("/main/home".into()),
                )
                .center()
                .title(&app_name)
                .fullscreen(false)
                .resizable(false)
                .maximized(false)
                .inner_size(360., 640.);

                #[cfg(target_os = "macos")]
                {
                    use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

                    let mut menu = Menu::new();
                    menu = menu.add_submenu(Submenu::new(
                        app_name.clone(),
                        Menu::new()
                            .add_item(CustomMenuItem::new("about", "About MirrorX"))
                            .add_native_item(MenuItem::Separator)
                            .add_item(CustomMenuItem::new("quit", "Quit")),
                    ));

                    builder
                        .menu(menu)
                        .hidden_title(true)
                        .title_bar_style(tauri::TitleBarStyle::Overlay)
                        .build()
                }

                #[cfg(target_os = "windows")]
                {
                    builder.decorations(false).transparent(true).build()
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::config::config_init,
            command::config::config_domain_get,
            command::config::config_domain_get_by_name,
            command::config::config_domain_get_id_and_names,
            command::config::config_domain_create,
            command::config::config_domain_delete,
            command::config::config_domain_list,
            command::config::config_domain_update,
            command::config::config_language_get,
            command::config::config_language_set,
            command::config::config_theme_get,
            command::config::config_theme_set,
            command::config::config_history_get,
            command::lan::lan_init,
            command::lan::lan_connect,
            command::lan::lan_nodes_list,
            command::lan::lan_nodes_search,
            command::lan::lan_discoverable_get,
            command::lan::lan_discoverable_set,
            command::signaling::signaling_connect,
            command::signaling::signaling_visit,
            command::file_manager::file_manager_visit_remote,
            command::file_manager::file_manager_visit_local,
            command::file_manager::file_manager_send_file,
            command::file_manager::file_manager_download_file,
            command::file_manager::file_manager_query_transferred_bytes_count,
            command::utility::utility_generate_random_password,
            command::utility::utility_detect_os_platform,
            command::utility::utility_enum_graphics_cards,
            command::utility::utility_hide_macos_zoom_button,
        ])
        .build(tauri::generate_context!())?;

    Ok(app)
}

fn init_component(app: &App) -> anyhow::Result<()> {
    // init logger
    let log_dir = app
        .path_resolver()
        .app_log_dir()
        .ok_or(anyhow::anyhow!("resolve app log dir failed"))?;

    let appender = tracing_appender::rolling::daily(&log_dir, "mirrorx.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    let console_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stderr);

    tracing_subscriber::Registry::default()
        .with(EnvFilter::from("info,tao=info"))
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!(path = ?log_dir, "initialize logger");

    // init config db
    let config_dir = app
        .path_resolver()
        .app_config_dir()
        .ok_or(anyhow::anyhow!("resolve app config dir failed"))?;

    std::fs::create_dir_all(&config_dir)?;
    let path = config_dir.join("mirrorx.db");
    let storage = LocalStorage::new(&path)?;
    app.manage(storage);
    tracing::info!(?path, "initialize config db");

    Ok(())
}
