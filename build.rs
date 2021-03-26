#[cfg(windows)]
extern crate windres;

#[cfg(windows)]
fn main() {
    windres::Build::new().compile("res/res.rc").unwrap();
}

use std::process::Command;

#[cfg(unix)]
fn main() {
    Command::new("sh")
        .args(&["-c", "cd res && glib-compile-resources res.xml"])
        .output()
        .unwrap();
}