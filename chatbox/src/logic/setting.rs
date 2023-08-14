use crate::audio;
use crate::config;
use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use crate::util::{self, translator::tr};
use log::warn;
use slint::{ComponentHandle, VecModel, Weak};
use std::rc::Rc;

pub fn init(ui: &AppWindow) {
    let ui_cancel = ui.as_weak();
    let ui_ok = ui.as_weak();
    let cache_clean_btn = ui.as_weak();

    init_setting_dialog(ui.as_weak());

    ui.global::<Logic>().on_setting_cancel(move || {
        init_setting_dialog(ui_cancel.clone());
    });

    ui.global::<Logic>().on_clean_audio_cache(move || {
        let ui = cache_clean_btn.unwrap();

        match util::fs::remove_dir_files(&config::audio_path()) {
            Err(e) => ui.global::<Logic>().invoke_show_message(
                slint::format!("{}{:?}", tr("清除缓存失败") + "！", e),
                "warning".into(),
            ),
            _ => {
                let mut cache = ui.global::<Store>().get_cache();
                cache.audio = "0M".into();
                ui.global::<Store>().set_cache(cache);
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}", tr("清除缓存成功") + "！"),
                    "success".into(),
                );
            }
        }
    });

    ui.global::<Logic>().on_setting_ok(move |setting_config| {
        let ui = ui_ok.unwrap();
        let mut config = config::config();

        config.ui.font_size = setting_config
            .ui
            .font_size
            .to_string()
            .parse()
            .unwrap_or(18);
        config.ui.font_family = setting_config
            .ui
            .font_family
            .to_string();
        config.ui.win_width = setting_config
            .ui
            .win_width
            .to_string()
            .parse()
            .unwrap_or(1200);
        config.ui.win_height = setting_config
            .ui
            .win_height
            .to_string()
            .parse()
            .unwrap_or(800);

        config.ui.language = setting_config.ui.language.to_string();

        config.socks5.openai = setting_config.proxy.openai;
        config.socks5.azure = setting_config.proxy.azure;
        config.socks5.url = setting_config.proxy.url.to_string();
        config.socks5.port = setting_config
            .proxy
            .port
            .to_string()
            .parse()
            .unwrap_or(1080);
        config.openai.api_key = setting_config.chat.openai.api_key.to_string();
        config.openai.chat.url = setting_config.chat.openai.chat.url.to_string();
        config.openai.chat.max_tokens = setting_config
            .chat
            .openai
            .chat
            .max_tokens
            .to_string()
            .parse::<f32>()
            .unwrap_or(1024.0) as i32;

        config.openai.chat.max_tokens_16k = setting_config
            .chat
            .openai
            .chat
            .max_tokens_16k
            .to_string()
            .parse::<f32>()
            .unwrap_or(10240.0) as i32;

        config.openai.chat.temperature =
            setting_config.chat.openai.chat.temperature.round() / 100.0;
        config.openai.chat.frequency_penalty =
            setting_config.chat.openai.chat.frequency_penalty.round() / 100.0;
        config.openai.chat.presence_penalty =
            setting_config.chat.openai.chat.presence_penalty.round() / 100.0;

        config.openai.chat.context_length = setting_config.chat.openai.chat.context_length.to_string();

        config.azureai.api_key = setting_config.chat.azure.api_key.to_string();
        config.azureai.chat.url = setting_config.chat.azure.chat.url.to_string();
        config.azureai.chat.max_tokens = setting_config
            .chat
            .azure
            .chat
            .max_tokens
            .to_string()
            .parse::<f32>()
            .unwrap_or(1024.0) as i32;

        config.azureai.chat.temperature =
            setting_config.chat.azure.chat.temperature.round() / 100.0;
        config.azureai.chat.frequency_penalty =
            setting_config.chat.azure.chat.frequency_penalty.round() / 100.0;
        config.azureai.chat.presence_penalty =
            setting_config.chat.azure.chat.presence_penalty.round() / 100.0;

        config.audio.region = setting_config.audio.region.to_string();
        config.audio.api_key = setting_config.audio.api_key.to_string();
        config.audio.current_input_device = setting_config.audio.current_input_device.to_string();
        config.audio.current_output_device = setting_config.audio.current_output_device.to_string();
        config.audio.speech_language = setting_config.audio.speech_language.to_string();
        config.audio.is_auto_v2t = setting_config.audio.is_auto_v2t;
        config.audio.is_auto_play_record = setting_config.audio.is_auto_play_record;
        config.audio.output_volume = setting_config.audio.output_volume.round() / 100.0;

        match config::save(config) {
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}{:?}", tr("保存失败") + "！", e),
                    "warning".into(),
                );
            }
            _ => {
                init_setting_dialog(ui.as_weak());
                ui.global::<Logic>()
                    .invoke_show_message((tr("保存成功") + "!").into(), "success".into());
            }
        }
    });
}

fn init_setting_dialog(ui: Weak<AppWindow>) {
    let ui = ui.unwrap();
    let ui_config = config::ui();
    let socks5_config = config::socks5();
    let openai_config = config::openai();
    let azureai_config = config::azureai();
    let audio_config = config::audio();

    let input_devices: VecModel<slint::SharedString> = VecModel::default();
    input_devices.extend(
        audio::record::input_devices_name()
            .into_iter()
            .map(|s| s.into())
            .collect::<Vec<_>>(),
    );
    ui.global::<Store>()
        .set_input_audio_devices(Rc::new(input_devices).into());

    let output_devices: VecModel<slint::SharedString> = VecModel::default();
    output_devices.extend(
        audio::play::output_devices_name()
            .into_iter()
            .map(|s| s.into())
            .collect::<Vec<_>>(),
    );
    ui.global::<Store>()
        .set_output_audio_devices(Rc::new(output_devices).into());

    let mut setting_dialog = ui.global::<Store>().get_setting_dialog_config();
    setting_dialog.ui.font_size = slint::format!("{}", ui_config.font_size);
    setting_dialog.ui.font_family = ui_config.font_family.into();
    setting_dialog.ui.win_width = slint::format!("{}", ui_config.win_width);
    setting_dialog.ui.win_height = slint::format!("{}", ui_config.win_height);
    setting_dialog.ui.language = ui_config.language.into();

    setting_dialog.proxy.openai = socks5_config.openai;
    setting_dialog.proxy.azure = socks5_config.azure;
    setting_dialog.proxy.url = socks5_config.url.into();
    setting_dialog.proxy.port = slint::format!("{}", socks5_config.port);

    setting_dialog.chat.openai.api_key = openai_config.api_key.into();
    setting_dialog.chat.openai.chat.url = openai_config.chat.url.into();
    setting_dialog.chat.openai.chat.max_tokens =
        slint::format!("{}", openai_config.chat.max_tokens);
    setting_dialog.chat.openai.chat.max_tokens_16k =
        slint::format!("{}", openai_config.chat.max_tokens_16k);
    setting_dialog.chat.openai.chat.temperature = openai_config.chat.temperature * 100.0;
    setting_dialog.chat.openai.chat.frequency_penalty =
        openai_config.chat.frequency_penalty * 100.0;
    setting_dialog.chat.openai.chat.presence_penalty = openai_config.chat.presence_penalty * 100.0;
    setting_dialog.chat.openai.chat.context_length = openai_config.chat.context_length.into();

    setting_dialog.chat.azure.api_key = azureai_config.api_key.into();
    setting_dialog.chat.azure.chat.url = azureai_config.chat.url.into();
    setting_dialog.chat.azure.chat.max_tokens =
        slint::format!("{}", azureai_config.chat.max_tokens);
    setting_dialog.chat.azure.chat.temperature = azureai_config.chat.temperature * 100.0;
    setting_dialog.chat.azure.chat.frequency_penalty =
        azureai_config.chat.frequency_penalty * 100.0;
    setting_dialog.chat.azure.chat.presence_penalty = azureai_config.chat.presence_penalty * 100.0;

    setting_dialog.audio.region = audio_config.region.into();
    setting_dialog.audio.api_key = audio_config.api_key.into();
    setting_dialog.audio.current_input_device = audio_config.current_input_device.into();
    setting_dialog.audio.current_output_device = audio_config.current_output_device.into();
    setting_dialog.audio.speech_language = audio_config.speech_language.into();
    setting_dialog.audio.is_auto_v2t = audio_config.is_auto_v2t;
    setting_dialog.audio.is_auto_play_record = audio_config.is_auto_play_record;
    setting_dialog.audio.output_volume = audio_config.output_volume * 100.0;

    ui.global::<Store>()
        .set_setting_dialog_config(setting_dialog);

    let mut cache = ui.global::<Store>().get_cache();
    match util::fs::dir_size(&config::audio_path()) {
        Ok(size) => {
            cache.audio = size.into();
            ui.global::<Store>().set_cache(cache);
        }
        Err(e) => warn!("get audio cache size failed. {:?}", e),
    }
}
