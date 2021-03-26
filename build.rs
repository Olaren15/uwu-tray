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