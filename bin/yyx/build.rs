
#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("yyx.ico");
  res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}