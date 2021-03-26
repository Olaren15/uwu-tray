#![windows_subsystem = "windows"]

use tray_item::TrayItem;
use clipboard::{ClipboardProvider, ClipboardContext};

#[cfg(target_os = "linux")]
use gtk::IconThemeExt;

fn init() {
    #[cfg(target_os = "linux")]
        {
            gtk::init().unwrap();
            let resources_bytes = include_bytes!("../res/res.gresource");
            let resource_data = glib::Bytes::from(&resources_bytes[..]);
            let res = gio::Resource::new_from_data(&resource_data).unwrap();
            gio::resources_register(&res);
            gtk::IconTheme::get_default().unwrap().add_resource_path("/dev/olaren/uwu-tray");
        }
}

fn quit() {
    #[cfg(target_os = "windows")]
        std::process::exit(0);

    #[cfg(target_os = "linux")]
        gtk::main_quit();
}

fn main_loop() {
    #[cfg(target_os = "windows")]
        std::io::stdin().read_line(&mut String::new()).unwrap();

    #[cfg(target_os = "linux")]
        gtk::main();
}

fn main() {
    init();

    let mut tray = TrayItem::new("uwu", "uwu").expect("Failed to create tray icon");

    tray.add_menu_item("uwuify", || {
        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

        if let Ok(contents) = clipboard.get_contents() {
            clipboard.set_contents(uwuifier::uwuify_str_sse(contents.as_str())).unwrap();
        }
    }).unwrap();

    tray.add_menu_item("Quit", || {
        quit();
    }).unwrap();

    main_loop();
}