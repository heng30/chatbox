use crate::slint_generatedAppWindow::{AppWindow, Logic};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.global::<Logic>().on_hide_window(move || {
        let ui = ui_handle.unwrap();
        if let Err(e) = ui.window().hide() {
            ui.global::<Logic>().invoke_show_message(
                slint::format!("隐藏程序失败！原因：{:?}", e),
                "warning".into(),
            );
        }
    });
}
