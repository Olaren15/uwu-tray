#[cfg(windows)]
extern crate windres;

#[cfg(windows)]
fn main() {
    windres::Build::new().compile("res/res.rc").unwrap();
}

#[cfg(unix)]
fn main() {}