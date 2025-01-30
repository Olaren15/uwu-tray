#![windows_subsystem = "windows"]

use anyhow::{anyhow, Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::string::ToString;
use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItemBuilder};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

const ICON_BYTES: &[u8] = include_bytes!("../res/uwu.png");
const UWUIFY_MENU_ID: &str = "uwuify";
const QUIT_MENU_ID: &str = "quit";

struct App {
    tray_icon: Option<TrayIcon>,
    clipboard_context: ClipboardContext,
}

impl App {
    fn new() -> Result<App> {
        let clipboard_context = ClipboardContext::new()
            .map_err(|err| anyhow!("{}", err))
            .context("Cannot create clipboard context")?;

        let app = App {
            tray_icon: None,
            clipboard_context,
        };

        Ok(app)
    }

    fn new_tray_icon(&self) -> Result<TrayIcon> {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(ICON_BYTES)?.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)?;

        let uwuify_item = MenuItemBuilder::new()
            .id(MenuId::new(UWUIFY_MENU_ID))
            .text("uwuify".to_string())
            .enabled(true)
            .build();
        
        let quit_item = MenuItemBuilder::new()
            .id(MenuId::new(QUIT_MENU_ID))
            .text("Quit".to_string())
            .enabled(true)
            .build();

        let menu = Menu::new();
        menu.append(&uwuify_item)?;
        menu.append(&quit_item)?;

        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("uwu-tray")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .with_title("uwu-tray")
            .build()?;

        Ok(tray_icon)
    }
}

impl ApplicationHandler<MenuEvent> for App {
    fn new_events(&mut self, _: &ActiveEventLoop, cause: StartCause) {
        // We create the icon once the event loop is actually running
        // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
        if cause == StartCause::Init {
            #[cfg(not(target_os = "linux"))]
            {
                self.tray_icon = Some(self.new_tray_icon().expect("Cannot create tray icon"))
            }

            // We have to request a redraw here to have the icon actually show up.
            // Winit only exposes a redraw method on the Window so we use core-foundation directly.
            #[cfg(target_os = "macos")]
            unsafe {
                use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};

                let rl = CFRunLoopGetMain().unwrap();
                CFRunLoopWakeUp(&rl);
            }
        }
    }

    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: MenuEvent) {
        match event.id.0.as_str() {
            UWUIFY_MENU_ID => {
                self.clipboard_context
                    .get_contents()
                    .map(|contents| uwuifier::uwuify_str_sse(contents.as_str()))
                    .and_then(|uwuified| self.clipboard_context.set_contents(uwuified))
                    .expect("Cannot uwuify");
            }
            QUIT_MENU_ID => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
}

fn main() -> Result<()> {
    let event_loop = EventLoop::<MenuEvent>::with_user_event().build()?;

    let menu_event_proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        menu_event_proxy.send_event(event).ok();
    }));

    let _menu_channel = MenuEvent::receiver();

    let mut app = App::new()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}
