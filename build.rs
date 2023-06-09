// extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("image/img.ico");
        res.compile().unwrap();
    }
    slint_build::compile("ui/appwindow.slint").unwrap();
}
