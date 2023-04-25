use crate::slint_generatedAppWindow::{AppWindow, Store};
use crate::version::VERSION;
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let mut about = ui.global::<Store>().get_about_dialog();
    about.title = slint::format!(
        "ChatBox {}",
        if VERSION.is_empty() {
            "v1.0.0"
        } else {
            VERSION
        }
    );
    ui.global::<Store>().set_about_dialog(about);
}
