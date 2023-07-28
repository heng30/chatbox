use crate::slint_generatedAppWindow::{AppWindow, Logic};
use crate::util::translator::tr;
use slint::ComponentHandle;
use crate::audio::azure;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.global::<Logic>().on_hide_window(move || {
        let ui = ui_handle.unwrap();
        if let Err(e) = ui.window().hide() {
            ui.global::<Logic>().invoke_show_message(
                slint::format!("{}:{:?}", tr("隐藏程序失败") + "！" + &tr("原因"), e),
                "warning".into(),
            );
        }
    });

    ui.global::<Logic>().on_stop_audio_play(move || {
        azure::stop_play();
    });
}
