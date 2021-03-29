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

#[cfg(target_os = "macos")]
// All in all binary produced is sufficient to run on macOS
// However, if you want to bundle macOS Application, it should happend post-build (once cargo
// generated binaries)
// And atm only option here is to manually run `bash bundle_macos.sh`
fn main() {}
