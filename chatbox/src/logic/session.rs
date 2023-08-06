use super::{chat, chatcache};
use crate::db;
use crate::db::data::{SessionChats, SessionConfig};
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, ChatSession, Logic, Store};
use crate::util::translator::tr;
#[allow(unused)]
use log::{debug, warn};
use slint::{ComponentHandle, Model, ModelExt, ModelRc, VecModel, Weak};
use std::cmp::Ordering;
use std::rc::Rc;
use uuid::Uuid;

const DEFAULT_SESSION_UUID: &str = "default-session-uuid";

fn init_db_default_session(ui: &AppWindow) {
    for session in ui.global::<Store>().get_chat_sessions().iter() {
        let uuid = session.uuid.to_string();

        match db::session::is_exist(&uuid) {
            Ok(exist) => {
                if exist {
                    continue;
                }
            }
            Err(e) => warn!("{:?}", e),
        }

        let config_json = match serde_json::to_string(&SessionConfig::from(&session)) {
            Ok(config) => config,
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("设置默认会话库失败") + "!" + &tr("原因"), e),
                    "warning".into(),
                );
                return;
            }
        };

        if let Err(e) = db::session::insert(uuid, config_json, "".to_string()) {
            ui.global::<Logic>().invoke_show_message(
                slint::format!(
                    "{}: {:?}",
                    tr("保存默认会话到数据库失败") + "!" + &tr("原因"),
                    e
                ),
                "warning".into(),
            );
            return;
        }
    }
}

fn init_session(ui: &AppWindow) {
    match db::session::select_all() {
        Ok(items) => {
            let sessions = VecModel::default();

            for item in items.into_iter() {
                let config = item.1;
                let chats = item.2;

                let mut chat_session = ChatSession {
                    uuid: item.0.into(),
                    ..Default::default()
                };

                match serde_json::from_str::<SessionConfig>(&config) {
                    Ok(sc) => {
                        chat_session.is_mark = sc.is_mark;
                        chat_session.use_history = sc.use_history;
                        chat_session.icon_index = sc.icon_index;
                        chat_session.name = sc.name.into();
                        chat_session.system_prompt = sc.system_prompt.into();
                        chat_session.api_model = sc.api_model.into();
                        chat_session.shortcut_instruction = sc.shortcut_instruction.into();
                    }
                    Err(e) => {
                        warn!("{:?}", e);
                        continue;
                    }
                }

                if !chats.is_empty() {
                    match serde_json::from_str::<SessionChats>(&chats) {
                        Ok(sc) => {
                            let chat_items = VecModel::default();
                            for citem in sc.chats.into_iter() {
                                chat_items.push(ChatItem {
                                    uuid: citem.uuid.into(),
                                    utext: citem.utext.into(),
                                    btext: citem.btext.as_str().into(),
                                    timestamp: citem.timestamp.into(),
                                    etext: "".into(),
                                    is_mark: citem.is_mark,
                                    btext_items: chat::parse_chat_text(citem.btext.as_str()).into(),
                                })
                            }

                            chat_session.chat_items = Rc::new(chat_items).into();
                        }
                        Err(e) => {
                            warn!("{:?}", e);
                            continue;
                        }
                    }
                }
                sessions.push(chat_session);
            }

            let sessions = sessions.sort_by(|a, b| -> Ordering {
                if a.uuid == DEFAULT_SESSION_UUID {
                    Ordering::Less
                } else if b.uuid == DEFAULT_SESSION_UUID {
                    Ordering::Greater
                } else if a.is_mark && b.is_mark {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                } else if a.is_mark && !b.is_mark {
                    Ordering::Less
                } else if !a.is_mark && b.is_mark {
                    Ordering::Greater
                } else {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                }
            });

            if sessions.row_count() > 0 {
                ui.global::<Store>()
                    .set_session_datas(sessions.row_data(0).unwrap().chat_items);
                ui.global::<Logic>().invoke_show_session_archive_list(
                    sessions.row_data(0).unwrap().uuid,
                    ui.get_archive_search_text(),
                );
            }

            ui.global::<Store>()
                .set_chat_sessions(Rc::new(sessions).into());
        }
        Err(e) => {
            warn!("{:?}", e);
        }
    }
}

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    let ui_delete_handle = ui.as_weak();
    let ui_set_edit_handle = ui.as_weak();
    let ui_reset_edit_handle = ui.as_weak();
    let ui_save_edit_handle = ui.as_weak();
    let ui_reset_handle = ui.as_weak();
    let ui_mark_handle = ui.as_weak();
    let ui_switch_handle = ui.as_weak();
    let ui_switch_shortcut_inst_handle = ui.as_weak();
    let ui_copy_handle = ui.as_weak();
    let ui_save_chats_handle = ui.as_weak();

    init_db_default_session(ui);
    init_session(ui);

    ui.global::<Logic>().on_handle_new_session(move |config| {
        let ui = ui_handle.unwrap();
        let mut sessions: Vec<ChatSession> =
            ui.global::<Store>().get_chat_sessions().iter().collect();

        let cs = ChatSession {
            name: config.name,
            system_prompt: config.system_prompt,
            use_history: config.use_history,
            api_model: config.api_model,
            shortcut_instruction: config.shortcut_instruction,
            icon_index: config.icon_index,
            uuid: Uuid::new_v4().to_string().into(),
            ..Default::default()
        };

        let config_json = match serde_json::to_string(&db::data::SessionConfig::from(&cs)) {
            Ok(config) => config,
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("保存到数据库失败") + "！" + &tr("原因"), e),
                    "warning".into(),
                );
                return;
            }
        };

        if let Err(e) = db::session::insert(cs.uuid.to_string(), config_json, "".to_string()) {
            ui.global::<Logic>().invoke_show_message(
                slint::format!("{}: {:?}", tr("保存到数据库失败") + "！" + &tr("原因"), e),
                "warning".into(),
            );
            return;
        }

        sessions.push(cs);
        let sessions_model = Rc::new(VecModel::from(sessions));
        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());
        ui.global::<Logic>()
            .invoke_show_message((tr("新建成功") + "！").into(), "success".into());
    });

    ui.global::<Logic>().on_delete_session(move |uuid| {
        let ui = ui_delete_handle.unwrap();

        if uuid == DEFAULT_SESSION_UUID {
            ui.global::<Logic>()
                .invoke_show_message((tr("不允许删除默认会话") + "!").into(), "warning".into());
            return;
        }

        let sessions: Vec<ChatSession> = ui
            .global::<Store>()
            .get_chat_sessions()
            .iter()
            .filter(|x| x.uuid != uuid)
            .collect();

        if let Err(e) = db::session::delete(uuid.to_string()) {
            ui.global::<Logic>().invoke_show_message(
                slint::format!("{}: {:?}", tr("删除会话失败") + "!" + &tr("原因"), e),
                "warning".into(),
            );
            return;
        }

        ui.global::<Logic>()
            .invoke_delete_session_archives(ui.global::<Store>().get_current_session_uuid());

        ui.global::<Store>()
            .set_current_session_uuid(DEFAULT_SESSION_UUID.into());

        ui.global::<Logic>().invoke_show_session_archive_list(
            DEFAULT_SESSION_UUID.into(),
            ui.get_archive_search_text(),
        );

        let sessions_model = Rc::new(VecModel::from(sessions));
        if sessions_model.row_count() > 0 {
            ui.global::<Store>()
                .set_session_datas(sessions_model.row_data(0).unwrap().chat_items);
        }

        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());
        ui.global::<Logic>()
            .invoke_show_message((tr("删除会话成功") + "!").into(), "success".into());
    });

    ui.global::<Logic>().on_reset_current_session(move || {
        let ui = ui_reset_handle.unwrap();
        ui.global::<Store>().set_session_datas(ModelRc::default());

        ui.global::<Logic>()
            .invoke_show_message((tr("重置成功") + "!").into(), "success".into());
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

        let mut is_mark = false;
        for cs in sessions.iter() {
            if cs.uuid != uuid {
                continue;
            }

            is_mark = cs.is_mark;

            match serde_json::to_string(&SessionConfig::from(cs)) {
                Ok(config) => {
                    if let Err(e) = db::session::update(uuid.to_string(), Some(config), None) {
                        ui.global::<Logic>().invoke_show_message(
                            slint::format!(
                                "{}: {:?}",
                                tr("保存到数据库失败") + "！" + &tr("原因"),
                                e
                            ),
                            "warning".into(),
                        );
                        return;
                    }
                    break;
                }
                Err(e) => {
                    ui.global::<Logic>().invoke_show_message(
                        slint::format!("{}: {:?}", tr("保存到数据库失败") + "！" + &tr("原因"), e),
                        "warning".into(),
                    );
                    return;
                }
            };
        }

        let sessions_model = Rc::new(VecModel::from(sessions));
        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());

        if is_mark {
            ui.global::<Logic>()
                .invoke_show_message((tr("收藏成功") + "！").into(), "success".into());
        } else {
            ui.global::<Logic>()
                .invoke_show_message((tr("取消收藏成功") + "！").into(), "success".into());
        }
    });

    ui.global::<Logic>().on_set_edit_session(move |uuid| {
        let ui = ui_set_edit_handle.unwrap();
        let sessions: Vec<ChatSession> = ui
            .global::<Store>()
            .get_chat_sessions()
            .iter()
            .filter(|x| x.uuid == uuid)
            .collect();

        // debug!("{:?}", sessions);
        if sessions.is_empty() {
            return;
        }

        ui.set_session_name(sessions[0].name.clone());
        ui.set_session_system_prompt(sessions[0].system_prompt.clone());
        ui.set_session_api_model(sessions[0].api_model.clone());
        ui.set_session_shortcut_instruction(sessions[0].shortcut_instruction.clone());
        ui.set_session_use_history(sessions[0].use_history);
        ui.set_session_icon_index(sessions[0].icon_index);
    });

    ui.global::<Logic>().on_reset_edit_session(move || {
        let ui = ui_reset_edit_handle.unwrap();
        ui.set_session_name("".into());
        ui.set_session_system_prompt("".into());
        ui.set_session_shortcut_instruction("".into());
        ui.set_session_use_history(false);
        ui.set_session_icon_index(0);
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
                            shortcut_instruction: config.shortcut_instruction.clone(),
                            icon_index: config.icon_index,
                            use_history: config.use_history,
                            ..x
                        }
                    }
                })
                .collect();

            for session in sessions.iter() {
                if session.uuid == uuid {
                    match serde_json::to_string(&SessionConfig::from(session)) {
                        Ok(config) => {
                            if let Err(e) =
                                db::session::update(uuid.to_string(), Some(config), None)
                            {
                                ui.global::<Logic>().invoke_show_message(
                                    slint::format!(
                                        "{}: {:?}",
                                        tr("保存会话失败") + "！" + &tr("原因"),
                                        e
                                    ),
                                    "warning".into(),
                                );
                                return;
                            }
                            break;
                        }
                        Err(e) => {
                            ui.global::<Logic>().invoke_show_message(
                                slint::format!(
                                    "{}: {:?}",
                                    tr("保存会话配置失败") + "！" + &tr("原因"),
                                    e
                                ),
                                "warning".into(),
                            );
                            return;
                        }
                    };
                }
            }

            let sessions_model = Rc::new(VecModel::from(sessions));
            ui.global::<Store>()
                .set_chat_sessions(sessions_model.into());
            ui.global::<Logic>()
                .invoke_show_message((tr("保存会话配置成功") + "!").into(), "success".into());
        });

    ui.global::<Logic>()
        .on_switch_session(move |old_uuid, new_uuid| {
            if old_uuid == new_uuid || new_uuid.is_empty() {
                return;
            }

            let ui = ui_switch_handle.unwrap();
            let chat_items = ui.global::<Store>().get_session_datas();
            let sessions = ui.global::<Store>().get_chat_sessions();
            let chats_viewport_y = ui.get_chats_viewport_y();

            let mut index = 0;
            for (row, session) in sessions.iter().enumerate() {
                if session.uuid == old_uuid {
                    ui.global::<Store>().get_chat_sessions().set_row_data(
                        row,
                        ChatSession {
                            chats_viewport_y,
                            chat_items: chat_items.clone(),
                            ..session
                        },
                    );

                    index += 1;
                } else if session.uuid == new_uuid {
                    ui.global::<Store>()
                        .set_session_datas(session.chat_items.clone());

                    // join the cache text that recieved in background
                    let row_count = ui.global::<Store>().get_session_datas().row_count();
                    if row_count > 0 {
                        let last_row_index = row_count - 1;
                        if let Some(item) = ui
                            .global::<Store>()
                            .get_session_datas()
                            .row_data(last_row_index)
                        {
                            if let Some((cuuid, text)) = chatcache::get_cache(new_uuid.as_str()) {
                                // debug!("xxx - {} - {} - {}", &cuuid, &item.uuid, &text);
                                if item.uuid == cuuid {
                                    let btext = if item.btext == chat::LOADING_STRING {
                                        text.trim_start().into()
                                    } else {
                                        item.btext + &text
                                    };

                                    ui.global::<Store>().get_session_datas().set_row_data(
                                        last_row_index,
                                        ChatItem {
                                            btext: btext.clone(),
                                            btext_items: chat::parse_chat_text(btext.as_str())
                                                .into(),
                                            ..item
                                        },
                                    );
                                    ui.window().request_redraw();
                                }
                            }
                        }
                    }

                    ui.set_archive_search_text("".into());
                    ui.global::<Logic>().invoke_show_session_archive_list(
                        new_uuid.clone(),
                        ui.get_archive_search_text(),
                    );
                    ui.global::<Store>()
                        .set_previous_session_uuid(old_uuid.clone());
                    ui.global::<Store>()
                        .set_current_session_uuid(new_uuid.clone());
                    ui.invoke_archive_scroll_to_top();
                    ui.set_chats_viewport_y(session.chats_viewport_y);

                    index += 1;
                }

                if index == 2 {
                    break;
                }
            }
        });

    ui.global::<Logic>()
        .on_switch_session_shortcut_inst(move |current_uuid| {
            let ui = ui_switch_shortcut_inst_handle.unwrap();
            let mut question = ui.get_question();

            for session in ui.global::<Store>().get_chat_sessions().iter() {
                let shortcut_inst = session.shortcut_instruction;
                if shortcut_inst.is_empty() || !question.starts_with(shortcut_inst.as_str()) {
                    continue;
                }

                question = question
                    .trim_start_matches(shortcut_inst.as_str())
                    .trim_start()
                    .into();

                ui.global::<Logic>()
                    .invoke_switch_session(current_uuid, session.uuid);
                ui.set_question(question);
                return;
            }
        });

    ui.global::<Logic>().on_copy_session_chats(move |_uuid| {
        let ui = ui_copy_handle.unwrap();
        let mut chats = slint::SharedString::default();
        for item in ui.global::<Store>().get_session_datas().iter() {
            chats += &slint::format!("user:\n{}\n\nbot:\n{}\n\n", item.utext, item.btext);
        }

        ui.global::<Logic>().invoke_copy_to_clipboard(chats);
    });

    ui.global::<Logic>().on_save_session_chats(move |uuid| {
        let ui = ui_save_chats_handle.unwrap();
        let chats: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().collect();
        match serde_json::to_string::<SessionChats>(&SessionChats::from(&chats)) {
            Ok(text) => {
                if let Err(e) = db::session::update(uuid.to_string(), None, Some(text)) {
                    ui.global::<Logic>().invoke_show_message(
                        slint::format!("{}: {:?}", tr("保存会话失败") + "！" + &tr("原因"), e),
                        "warning".into(),
                    );
                } else {
                    ui.global::<Logic>()
                        .invoke_show_message((tr("保存会话成功") + "!").into(), "success".into());
                }
            }
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("保存会话失败") + "！" + &tr("原因"), e),
                    "warning".into(),
                );
            }
        }
    });
}

pub fn current_session_config(ui: Weak<AppWindow>) -> (String, String, String, bool) {
    let ui = ui.unwrap();
    let uuid = ui.global::<Store>().get_current_session_uuid();

    for session in ui.global::<Store>().get_chat_sessions().iter() {
        if session.uuid == uuid {
            // debug!("{:?}", session);
            return (
                session.system_prompt.into(),
                session.api_model.into(),
                session.shortcut_instruction.into(),
                session.use_history,
            );
        }
    }
    unreachable!("current session is not exist!");
}
