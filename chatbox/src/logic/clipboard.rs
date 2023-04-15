use crate::slint_generatedAppWindow::{AppWindow, Logic};
use clipboard::{ClipboardContext, ClipboardProvider};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_copy_to_clipboard(move |msg| {
        let ui = ui_handle.unwrap();

        let ctx: Result<ClipboardContext, _> = ClipboardProvider::new();
        match ctx {
            Ok(mut ctx) => match ctx.set_contents(msg.to_string()) {
                Err(e) => ui
                    .global::<Logic>()
                    .invoke_show_message(slint::format!("复制失败！{:?}", e), "warning".into()),
                _ => ui
                    .global::<Logic>()
                    .invoke_show_message("复制成功！".into(), "success".into()),
            },
            Err(e) => ui
                .global::<Logic>()
                .invoke_show_message(slint::format!("复制失败！{:?}", e), "warning".into()),
        }
    });
}
