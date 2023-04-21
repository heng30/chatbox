use crate::slint_generatedAppWindow::{AppWindow, ChatSession, Logic, Store};
#[allow(unused)]
use log::debug;
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::rc::Rc;
use uuid::Uuid;

const DEFAULT_SESSION_UUID: &str = "default-session-uuid";

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    let ui_delete_handle = ui.as_weak();
    let ui_set_edit_handle = ui.as_weak();
    let ui_save_edit_handle = ui.as_weak();
    let ui_reset_handle = ui.as_weak();
    let ui_mark_handle = ui.as_weak();
    let ui_switch_handle = ui.as_weak();

    ui.global::<Logic>().on_handle_new_session(move |config| {
        let ui = ui_handle.unwrap();
        let mut sessions: Vec<ChatSession> =
            ui.global::<Store>().get_chat_sessions().iter().collect();

        sessions.push(ChatSession {
            name: config.name,
            system_prompt: config.system_prompt,
            use_history: config.use_history,
            api_model: config.api_model,
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

    ui.global::<Logic>().on_set_edit_session(move |uuid| {
        let ui = ui_set_edit_handle.unwrap();
        let sessions: Vec<ChatSession> = ui
            .global::<Store>()
            .get_chat_sessions()
            .iter()
            .filter(|x| x.uuid == uuid)
            .collect();

        debug!("{:?}", sessions);
        if sessions.is_empty() {
            return;
        }

        ui.set_session_name(sessions[0].name.clone());
        ui.set_session_system_prompt(sessions[0].system_prompt.clone());
        ui.set_session_api_model(sessions[0].api_model.clone());
        ui.set_session_use_history(sessions[0].use_history);
    });

    ui.global::<Logic>()
        .on_save_edit_session(move |uuid, config| {
            let ui = ui_save_edit_handle.unwrap();
            let sessions: Vec<ChatSession> = ui
                .global::<Store>()
                .get_chat_sessions()
                .iter()
                .map(|x| {
                    if x.uuid != uuid {
                        x
                    } else {
                        ChatSession {
                            name: config.name.clone(),
                            system_prompt: config.system_prompt.clone(),
                            api_model: config.api_model.clone(),
                            use_history: config.use_history,
                            ..x
                        }
                    }
                })
                .collect();

            let sessions_model = Rc::new(VecModel::from(sessions));
            ui.global::<Store>()
                .set_chat_sessions(sessions_model.into());
        });

    ui.global::<Logic>()
        .on_switch_session(move |old_uuid, new_uuid| {
            let ui = ui_switch_handle.unwrap();
            let chat_items = ui.global::<Store>().get_session_datas();

            let sessions: Vec<ChatSession> = ui
                .global::<Store>()
                .get_chat_sessions()
                .iter()
                .map(|x| {
                    if x.uuid != old_uuid {
                        x
                    } else {
                        ChatSession {
                            chat_items: chat_items.clone(),
                            ..x
                        }
                    }
                })
                .collect();

            let sessions_model = Rc::new(VecModel::from(sessions));
            ui.global::<Store>()
                .set_chat_sessions(sessions_model.into());

            for session in ui.global::<Store>().get_chat_sessions().iter() {
                if session.uuid == new_uuid {
                    ui.global::<Store>().set_session_datas(session.chat_items.clone());
                }
            }
        });
}

pub fn current_session_config(ui: Weak<AppWindow>) -> (String, String, bool) {
    let ui = ui.unwrap();
    let uuid = ui.global::<Store>().get_current_session_uuid();

    for session in ui.global::<Store>().get_chat_sessions().iter() {
        if session.uuid == uuid {
            debug!("{:?}", session);
            return (
                session.system_prompt.into(),
                session.api_model.into(),
                session.use_history,
            );
        }
    }
    unreachable!("current session is not exist!");
}
