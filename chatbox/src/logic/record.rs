use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use crate::util::{self, qbox::QBox, translator::tr};
use crate::{audio, config};
use log::{debug, warn};
use slint::ComponentHandle;
use tokio::task::spawn;

static RECORD_FILENAME: &str = "record.wav";

pub fn init(ui: &AppWindow) {
    let ui_start_box = QBox::new(ui);
    let ui_play_box = QBox::new(ui);
    let ui_v2t_box = QBox::new(ui);
    let ui_stop = ui.as_weak();
    let ui_play = ui.as_weak();

    ui.global::<Logic>().on_start_audio_record(move || {
        spawn(async move {
            let audio_config = config::audio();
            let record_filepath = config::audio_path() + "/" + RECORD_FILENAME;
            let device_name = audio_config.current_input_device;

            if let Err(e) = slint::invoke_from_event_loop(move || {
                ui_start_box
                    .borrow()
                    .global::<Store>()
                    .set_is_audio_recording(true);
            }) {
                warn!("{:?}", e);
            }

            let is_auto_v2t = audio_config.is_auto_v2t;
            match audio::record::record(
                &device_name,
                &record_filepath,
                audio_config.max_recording_duration,
            ) {
                Err(e) => {
                    if let Err(e) = slint::invoke_from_event_loop(move || {
                        ui_start_box.borrow().global::<Logic>().invoke_show_message(
                            slint::format!("{}: {:?}", tr("录音失败") + "!", e),
                            "warning".into(),
                        );
                    }) {
                        warn!("{:?}", e);
                    }
                }
                _ => {
                    if is_auto_v2t {
                        if let Err(e) = slint::invoke_from_event_loop(move || {
                            ui_start_box
                                .borrow_mut()
                                .global::<Logic>()
                                .invoke_voice_to_text();
                        }) {
                            warn!("{:?}", e);
                        }
                    }
                }
            }

            audio::record::set_recording(false);

            if let Err(e) = slint::invoke_from_event_loop(move || {
                ui_start_box
                    .borrow()
                    .global::<Store>()
                    .set_is_audio_recording(false);
            }) {
                warn!("{:?}", e);
            }
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

    ui.global::<Logic>().on_voice_to_text(move || {
        ui_v2t_box
            .borrow()
            .global::<Logic>()
            .invoke_show_message(tr("正在将录音转换为文本...").into(), "info".into());

        spawn(async move {
            let record_filepath = config::audio_path() + "/" + RECORD_FILENAME;
            let audio_config = config::audio();
            match audio::azure::speech_to_text(&record_filepath, &audio_config.speech_language)
                .await
            {
                Err(e) => {
                    let estr = e.to_string();
                    if let Err(e) = slint::invoke_from_event_loop(move || {
                        ui_v2t_box.borrow().global::<Logic>().invoke_show_message(
                            slint::format!("{}: {:?}", tr("录音转文字失败") + "!", estr),
                            "warning".into(),
                        );
                    }) {
                        warn!("{:?}", e);
                    }
                }

                Ok(text) => match serde_json::from_str::<audio::data::Speech2Text>(&text) {
                    Ok(item) => {
                        debug!("{:?}", &item);
                        if item.recognition_status != "Success" {
                            if let Err(e) = slint::invoke_from_event_loop(move || {
                                ui_v2t_box.borrow().global::<Logic>().invoke_show_message(
                                    slint::format!("{}", tr("录音转文字失败") + "!"),
                                    "warning".into(),
                                );
                            }) {
                                warn!("{:?}", e);
                            }
                        }

                        if let Err(e) = slint::invoke_from_event_loop(move || {
                            let question = ui_v2t_box.borrow().get_question();
                            let question = if question.trim().is_empty() {
                                item.display_text.into()
                            } else {
                                question + &item.display_text
                            };
                            ui_v2t_box.borrow().set_question(question);
                        }) {
                            warn!("{:?}", e);
                        }
                    }
                    Err(e) => {
                        let estr = e.to_string();
                        if let Err(e) = slint::invoke_from_event_loop(move || {
                            ui_v2t_box.borrow().global::<Logic>().invoke_show_message(
                                slint::format!("{}: {:?}", tr("录音转文字失败") + "!", estr),
                                "warning".into(),
                            );
                        }) {
                            warn!("{:?}", e);
                        }
                    }
                },
            }
        });
    });
}
