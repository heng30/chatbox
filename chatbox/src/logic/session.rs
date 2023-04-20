use crate::slint_generatedAppWindow::{AppWindow, ChatSession, Logic, Store};
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::rc::Rc;
use uuid::Uuid;
#[allow(unused)]
use log::debug;

const DEFAULT_SESSION_UUID: &str = "default-session-uuid";

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    let ui_delete_handle = ui.as_weak();
    let ui_reset_handle = ui.as_weak();
    let ui_mark_handle = ui.as_weak();

    ui.global::<Logic>().on_handle_new_session(move |config| {
        let ui = ui_handle.unwrap();
        let mut sessions: Vec<ChatSession> =
            ui.global::<Store>().get_chat_sessions().iter().collect();

        sessions.push(ChatSession {
            name: config.name,
            system_prompt: config.system_prompt,
            uuid: Uuid::new_v4().to_string().into(),
            ..Default::default()
        });

        let sessions_model = Rc::new(VecModel::from(sessions));
        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());
    });

    ui.global::<Logic>().on_delete_session(move |uuid| {
        let ui = ui_delete_handle.unwrap();

        if uuid == DEFAULT_SESSION_UUID {
            ui.global::<Logic>()
                .invoke_show_message("不允许删除默认会话!".into(), "warning".into());
            return;
        }

        let sessions: Vec<ChatSession> = ui
            .global::<Store>()
            .get_chat_sessions()
            .iter()
            .filter(|x| x.uuid != uuid)
            .collect();

        let sessions_model = Rc::new(VecModel::from(sessions));
        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());

        if uuid == ui.global::<Store>().get_current_session_uuid() {
            ui.global::<Store>()
                .set_current_session_uuid(DEFAULT_SESSION_UUID.into());
        }
    });

    ui.global::<Logic>().on_reset_current_session(move || {
        let ui = ui_reset_handle.unwrap();
        ui.global::<Store>().set_session_datas(ModelRc::default());

        ui.global::<Logic>()
            .invoke_show_message("重置成功!".into(), "success".into());
    });

    ui.global::<Logic>().on_toggle_mark_session(move |uuid| {
        let ui = ui_mark_handle.unwrap();
        let sessions: Vec<ChatSession> = ui
            .global::<Store>()
            .get_chat_sessions()
            .iter()
            .map(|x| {
                if x.uuid != uuid {
                    x
                } else {
                    let mut m = x.clone();
                    m.is_mark = !x.is_mark;
                    m
                }
            })
            .collect();

        let sessions_model = Rc::new(VecModel::from(sessions));
        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());
    });
}

pub fn current_session_system_prompt(ui: Weak<AppWindow>) -> String {
    let ui = ui.unwrap();
    let uuid = ui.global::<Store>().get_current_session_uuid();

    let sessions: Vec<ChatSession> = ui
        .global::<Store>()
        .get_chat_sessions()
        .iter()
        .filter(|x| x.uuid == uuid)
        .collect();

    debug!("{:?}", sessions);
    if sessions.is_empty() {
        String::default()
    } else {
        sessions[0].system_prompt.clone().into()
    }
}
