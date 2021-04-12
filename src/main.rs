#![windows_subsystem = "windows"]

use clipboard::{ClipboardContext, ClipboardProvider};
use tray_item::TrayItem;

#[cfg(target_os = "linux")]
use gtk::IconThemeExt;

fn init() {
    #[cfg(target_os = "linux")]
    {
        gtk::init().unwrap();
        let resources_bytes = include_bytes!("../res/res.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::new_from_data(&resource_data).expect("Failed to initialise GTK");
        gio::resources_register(&res);
        gtk::IconTheme::get_default()
            .expect("Failed to get default IconTheme")
            .add_resource_path("/dev/olaren/uwu-tray");
    }
}

fn quit() {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    std::process::exit(0);

    #[cfg(target_os = "linux")]
    gtk::main_quit();
}

// suppress compilation warning on windows and linux
#[allow(unused_variables, unused_mut)]
fn main_loop(mut tray: TrayItem) {
    #[cfg(target_os = "windows")]
    loop {} // uses a lot of cpu, but it works

    #[cfg(target_os = "linux")]
    gtk::main();

    #[cfg(target_os = "macos")]
    tray.inner_mut().display();
}

fn main() {
    init();

    let mut tray = TrayItem::new("uwu", "uwu").expect("Failed to create tray icon");

    tray.add_menu_item("uwuify", || {
        let mut clipboard: ClipboardContext =
            ClipboardProvider::new().expect("Failed to create clipboard context");

        if let Ok(contents) = clipboard.get_contents() {
            clipboard
                .set_contents(uwuifier::uwuify_str_sse(contents.as_str()))
                .expect("Failed to set clipboard contents");
        }
    })
    .expect("Failed to add menu item");

    tray.add_menu_item("Quit", || {
        quit();
    })
    .expect("Failed to add menu item");

    main_loop(tray);
}
