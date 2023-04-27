use crate::config;
use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use crate::util::translator::tr;
use slint::{ComponentHandle, Weak};

pub fn init(ui: &AppWindow) {
    let ui_cancel = ui.as_weak();
    let ui_ok = ui.as_weak();
    init_setting_dialog(ui.as_weak());

    ui.global::<Logic>().on_setting_cancel(move || {
        init_setting_dialog(ui_cancel.clone());
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

        config.socks5.enabled = setting_config.proxy.enabled;
        config.socks5.url = setting_config.proxy.url.to_string();
        config.socks5.port = setting_config
            .proxy
            .port
            .to_string()
            .parse()
            .unwrap_or(1080);
        config.openai.api_key = setting_config.chat.openai.api_key.to_string();
        config.openai.chat.url = setting_config.chat.openai.chat.url.to_string();
        config.openai.chat.model = setting_config.chat.openai.chat.model.to_string();
        config.openai.chat.max_tokens = setting_config
            .chat
            .openai
            .chat
            .max_tokens
            .to_string()
            .parse::<f32>()
            .unwrap_or(1024.0) as i32;

        config.openai.chat.temperature =
            setting_config.chat.openai.chat.temperature.round() / 100.0;
        config.openai.chat.frequency_penalty =
            setting_config.chat.openai.chat.frequency_penalty.round() / 100.0;
        config.openai.chat.presence_penalty =
            setting_config.chat.openai.chat.presence_penalty.round() / 100.0;

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

    let mut setting_dialog = ui.global::<Store>().get_setting_dialog_config();
    setting_dialog.ui.font_size = slint::format!("{}", ui_config.font_size);
    setting_dialog.ui.win_width = slint::format!("{}", ui_config.win_width);
    setting_dialog.ui.win_height = slint::format!("{}", ui_config.win_height);
    setting_dialog.ui.language = ui_config.language.into();

    setting_dialog.proxy.enabled = socks5_config.enabled;
    setting_dialog.proxy.url = socks5_config.url.into();
    setting_dialog.proxy.port = slint::format!("{}", socks5_config.port);

    setting_dialog.chat.openai.api_key = openai_config.api_key.into();
    setting_dialog.chat.openai.chat.url = openai_config.chat.url.into();
    setting_dialog.chat.openai.chat.model = openai_config.chat.model.into();
    setting_dialog.chat.openai.chat.max_tokens =
        slint::format!("{}", openai_config.chat.max_tokens);
    setting_dialog.chat.openai.chat.temperature = openai_config.chat.temperature * 100.0;
    setting_dialog.chat.openai.chat.frequency_penalty =
        openai_config.chat.frequency_penalty * 100.0;
    setting_dialog.chat.openai.chat.presence_penalty = openai_config.chat.presence_penalty * 100.0;

    ui.global::<Store>()
        .set_setting_dialog_config(setting_dialog);
}
