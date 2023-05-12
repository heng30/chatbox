use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use crate::util::{self, qbox::QBox, translator::tr};
use crate::{audio, config};
use slint::ComponentHandle;
use tokio::task::spawn;

static RECORD_FILENAME: &str = "record.wav";

pub fn init(ui: &AppWindow) {
    let ui_start_box = QBox::new(ui);
    let ui_play_box = QBox::new(ui);
    let ui_stop = ui.as_weak();
    let ui_play = ui.as_weak();

    ui.global::<Logic>().on_start_audio_record(move || {
        spawn(async move {
            let audio_config = config::audio();
            let record_filepath = config::audio_path() + "/" + RECORD_FILENAME;
            let device_name = audio_config.current_input_device;

            ui_start_box
                .borrow()
                .global::<Store>()
                .set_is_audio_recording(true);

            if let Err(e) = audio::record::record(&device_name, &record_filepath, 300) {
                ui_start_box.borrow().global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("录音失败") + "!", e),
                    "warning".into(),
                );
            };

            audio::record::set_recording(false);
            ui_start_box
                .borrow()
                .global::<Store>()
                .set_is_audio_recording(false);
        });
    });

    ui.global::<Logic>().on_stop_audio_record(move || {
        let ui = ui_stop.unwrap();
        ui.global::<Logic>()
            .invoke_show_message(slint::format!("{}", tr("停止录音...")), "info".into());
        audio::record::set_recording(false);
        ui.global::<Store>().set_is_audio_recording(false);
    });

    ui.global::<Logic>().on_play_audio_record(move || {
        let ui = ui_play.unwrap();
        let record_filepath = config::audio_path() + "/" + RECORD_FILENAME;
        if util::fs::file_exist(&record_filepath) {
            audio::azure::play(ui_play_box, RECORD_FILENAME.to_string(), String::default())
        } else {
            ui.global::<Logic>().invoke_show_message(
                slint::format!("{}", tr("录音文件不存在") + "!"),
                "info".into(),
            );
        }
    });
}
