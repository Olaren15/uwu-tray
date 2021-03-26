#[cfg(windows)]
extern crate windres;

#[cfg(windows)]
fn main() {
    windres::Build::new().compile("res/res.rc").unwrap();
}

#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "linux")]
fn main() {
    Command::new("sh")
        .args(&["-c", "cd res && glib-compile-resources res.xml"])
        .output()
        .unwrap();
}

#[cfg(target_os="macos")]
use std::{fs, process::Command};

#[cfg(target_os="macos")]
fn main() {
    println!("Building uwu-tray MacOS .app package");
    Command::new("cargo").arg("build").arg("--release").arg("--package=uwu-tray");

    println!("packing uwu-tray.dmg");
    fs::create_dir_all("assets/macos/uwu-tray.app/Contents/MacOS").expect("Cannot create [MacOS] directory for MacOS app");
    fs::create_dir_all("assets/macos/uwu-tray.app/Contents/Resources").expect("Cannot create [Resources] directory for MacOS app");

    fs::copy("target/release/uwu-tray", "assets/macos/uwu-tray.app/Contents/MacOS/uwu-tray").expect("Cannot copy uwu-tray binary into MacOS app");
    fs::copy("res/uwu.icns",            "assets/macos/uwu-tray.app/Contents/Resources/AppIcon.icns").expect("Cannot copy icon.icns into Resources");
    fs::copy("res/Info.plist",          "assets/macos/uwu-tray.app/Contents/Info.plist").expect("Cannot copy Info.plist into MacOS package");
    
    Command::new("hdiutil").arg("create").arg("assets/uwu-tray.dmg").arg("-volname").arg("uwu-tray").arg("-srcfolder").arg("assets/macos").arg("-ov");
    println!("Package build is completed");
}
