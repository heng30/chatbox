use crate::config;
use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use slint::{ComponentHandle, Weak};

pub fn init(ui: &AppWindow) {
    let ui_cancel = ui.as_weak();
    init_setting_dialog(ui.as_weak());

    ui.global::<Logic>().on_setting_cancel(move || {
        init_setting_dialog(ui_cancel.clone());
    });
}

fn init_setting_dialog(ui: Weak<AppWindow>) {
    let ui = ui.unwrap();
    let ui_config = config::ui();
    let socks5_config = config::socks5();
    let openai_config = config::openai();

    let mut setting_dialog = ui.global::<Store>().get_setting_dialog_config();
    setting_dialog.ui.default_font_size = ui_config.font_size as f32;

    setting_dialog.proxy.enable = socks5_config.enable;
    setting_dialog.proxy.url = socks5_config.url.into();
    setting_dialog.proxy.port = socks5_config.port as i32;

    setting_dialog.chat.openai.api_key = openai_config.api_key.into();
    setting_dialog.chat.openai.chat.url = openai_config.chat.url.into();
    setting_dialog.chat.openai.chat.model = openai_config.chat.model.into();
    setting_dialog.chat.openai.chat.max_tokens = openai_config.chat.max_tokens;
    setting_dialog.chat.openai.chat.temperature = openai_config.chat.temperature * 100.0;
    setting_dialog.chat.openai.chat.frequency_penalty =
        openai_config.chat.frequency_penalty * 100.0;
    setting_dialog.chat.openai.chat.presence_penalty = openai_config.chat.presence_penalty * 100.0;

    ui.global::<Store>()
        .set_setting_dialog_config(setting_dialog);
}
