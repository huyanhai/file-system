#[allow(dead_code)]
pub enum ToolbarThickness {
    Thick,
    Medium,
    Thin,
}
use tauri::{Runtime, Window};

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self) {
        use cocoa::{appkit::NSWindow, base::YES};

        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;

            id.setTitlebarAppearsTransparent_(YES);
            // id.setMovableByWindowBackground_(YES);
        }
    }
}
