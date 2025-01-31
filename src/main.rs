#![windows_subsystem = "windows"]

use anyhow::{anyhow, Context, Result};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::string::ToString;
use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItemBuilder};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

const ICON_BYTES: &[u8] = include_bytes!("uwu.png");
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

    fn create_tray_icon(&mut self) -> Result<()> {
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

        self.tray_icon = Some(tray_icon);
        Ok(())
    }

    fn handle_menu_event(&mut self, menu_event: MenuEvent) -> bool {
        match menu_event.id.0.as_str() {
            UWUIFY_MENU_ID => {
                self.clipboard_context
                    .get_contents()
                    .map(|contents| uwuifier::uwuify_str_sse(contents.as_str()))
                    .and_then(|uwuified| self.clipboard_context.set_contents(uwuified))
                    .expect("Cannot uwuify");

                false
            }
            QUIT_MENU_ID => true,
            _ => false,
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::<MenuEvent>::with_user_event().build();

    let menu_event_proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        menu_event_proxy.send_event(event).ok();
    }));

    let _menu_channel = MenuEvent::receiver();

    let mut app = App::new()?;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                if let Err(_) = app.create_tray_icon() {
                    *control_flow = ControlFlow::Exit;
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

            Event::UserEvent(event) => {
                if app.handle_menu_event(event) {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
