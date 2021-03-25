#![windows_subsystem = "windows"]

use tray_item::TrayItem;
use clipboard::{ClipboardProvider, ClipboardContext};
use std::str;

#[inline(always)]
fn round_up(a: usize, b: usize) -> usize { (a + b - 1) / b * b }

fn main() {
    let mut tray = TrayItem::new("uwu", "uwu").expect("Failed to create tray icon");

    tray.add_menu_item("uwuify", || {
        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

        if let Ok(contents) = clipboard.get_contents() {
            let mut temp_bytes1 = [0u8; 1000000];
            let mut temp_bytes2 = [0u8; 1000000];

            let mut bytes = contents.as_bytes().to_owned();
            let len = bytes.len();
            bytes.resize(round_up(len, 16), 0);
            let res_bytes = uwuifier::uwu_ify_sse(&bytes, len, &mut temp_bytes1, &mut temp_bytes2);

            clipboard.set_contents(String::from(str::from_utf8(res_bytes).unwrap())).unwrap();
        }
        println!("uwu");
    }).unwrap();

    tray.add_menu_item("Quit", || {
        std::process::exit(0);
    }).unwrap();

    loop {}
}