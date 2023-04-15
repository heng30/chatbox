use crate::slint_generatedAppWindow::{AppWindow, Logic, MessageItem, Store};
use slint::ComponentHandle;
use slint::{Timer, TimerMode};

pub fn init(ui: &AppWindow) {
    let timer = Timer::default();
    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_show_message(move |msg, msg_type| {
        let ui = ui_handle.unwrap();

        if timer.running() {
            timer.stop();
        }

        ui.global::<Store>().set_message(MessageItem {
            text: msg,
            text_type: msg_type,
        });

        timer.start(
            TimerMode::SingleShot,
            std::time::Duration::from_secs(2),
            move || {
                ui.global::<Store>().set_message(MessageItem {
                    text: "".into(),
                    text_type: "".into(),
                });
            },
        );
    });
}
