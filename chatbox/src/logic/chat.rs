use super::chatcache;
use super::data::{HistoryChat, StreamTextItem};
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, CodeTextItem, Logic, Store};
use crate::util::{qbox::QBox, translator::tr};
use crate::{audio, azureai, config, openai, session, util};
#[allow(unused_imports)]
use log::{debug, warn};
use rand::Rng;
use slint::{ComponentHandle, Model, VecModel};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use tokio::task::spawn;
use uuid::Uuid;

pub const LOADING_STRING: &str = "Thinking...";

lazy_static! {
    static ref STOP_CHAT: Mutex<RefCell<HashMap<String, bool>>> =
        Mutex::new(RefCell::new(HashMap::new()));
}

pub fn set_stop_chat(suuid: String, is_stop: bool) {
    let item = STOP_CHAT.lock().unwrap();
    let mut item = item.borrow_mut();
    if is_stop {
        item.insert(suuid, true);
    } else {
        item.remove(&suuid);
    }
}

pub fn is_stop_chat(suuid: &str) -> bool {
    let item = STOP_CHAT.lock().unwrap();
    let item = item.borrow();
    item.contains_key(suuid)
}

async fn send_text(
    ui_box: QBox<AppWindow>,
    mut chats: HistoryChat,
) -> Result<(), Box<dyn std::error::Error>> {
    let item = chats.items.pop().unwrap();
    let question = item.utext;

    let suuid = ui_box
        .borrow()
        .global::<Store>()
        .get_current_session_uuid()
        .to_string();

    let (system_prompt, api_model, _, use_history) =
        session::current_session_config(ui_box.borrow().as_weak());

    if api_model.contains("ChatGPT") {
        let openai_chat = openai::OpenAIChat::make(
            system_prompt,
            question,
            if use_history {
                chats
            } else {
                HistoryChat::default()
            },
        );

        // debug!("{:?}", openai_chat);

        let api_model = if api_model.contains("chat-3.5-turbo") {
            "gpt-3.5-turbo".to_string()
        } else {
            return Err(anyhow::anyhow!("unknown api model: {}", api_model).into());
        };

        return openai::generate_text(openai_chat, api_model, suuid, item.uuid, move |sitem| {
            if let Err(e) = slint::invoke_from_event_loop(move || {
                stream_text(ui_box, sitem);
            }) {
                warn!("{:?}", e);
            }
        })
        .await;
    } else if api_model.contains("Azure") {
        let azureai_chat = azureai::AzureAIChat::make(
            system_prompt,
            question,
            if use_history {
                chats
            } else {
                HistoryChat::default()
            },
        );

        // debug!("{:?}", azureai_chat);

        let api_model = if api_model.contains("chat-35-turbo") {
            "gpt-35-turbo".to_string()
        } else {
            return Err(anyhow::anyhow!("unknown api model: {}", api_model).into());
        };

        return azureai::generate_text(azureai_chat, api_model, suuid, item.uuid, move |sitem| {
            if let Err(e) = slint::invoke_from_event_loop(move || {
                stream_text(ui_box, sitem);
            }) {
                warn!("{:?}", e);
            }
        })
        .await;
    }

    unreachable!("should not run in here");
}

fn stream_text(ui_box: QBox<AppWindow>, sitem: StreamTextItem) {
    let ui = ui_box.borrow();
    if sitem.finished {
        ui.window().request_redraw();
        return;
    }

    let text = match sitem.etext {
        Some(etext) => {
            ui.global::<Logic>().invoke_show_message(
                slint::format!("{}! {}: {}", tr("出错"), tr("原因"), etext),
                "warning".into(),
            );
            return;
        }
        _ => match sitem.text {
            Some(txt) => txt,
            _ => return,
        },
    };

    let suuid = ui_box
        .borrow()
        .global::<Store>()
        .get_current_session_uuid()
        .to_string();

    // debug!("yyy - {} - {} - {}", &suuid, &sitem.suuid, &sitem.uuid);
    if suuid != sitem.suuid {
        chatcache::update_cache(sitem.suuid, sitem.uuid.to_string(), text);
        return;
    }

    let rows = ui.global::<Store>().get_session_datas().row_count();
    if rows == 0 {
        set_stop_chat(suuid, true);
        return;
    }
    let current_row = rows - 1;

    if let Some(item) = ui
        .global::<Store>()
        .get_session_datas()
        .row_data(current_row)
    {
        if sitem.uuid != item.uuid.as_str() {
            return;
        }

        let btext = if item.btext == LOADING_STRING {
            text.trim_start().into()
        } else {
            item.btext.clone() + &text
        };

        ui.global::<Store>().get_session_datas().set_row_data(
            current_row,
            ChatItem {
                btext: btext.clone(),
                btext_items: parse_chat_text(btext.as_str()).into(),
                ..item
            },
        );

        if ui.get_is_chats_auto_scroll_down() {
            ui.invoke_chats_scroll_to_bottom();
        }

        if rand::thread_rng().gen_range(1..=10) > 7 {
            ui.window().request_redraw();
        }
    }
}

pub fn init(ui: &AppWindow) {
    let ui_box = QBox::new(ui);
    let ui_handle = ui.as_weak();
    let ui_retry_handle = ui.as_weak();
    let ui_delete_handle = ui.as_weak();
    let ui_mark_handle = ui.as_weak();
    let ui_audio_handle = ui.as_weak();
    let ui_remove_chats_first_handle = ui.as_weak();
    let ui_remove_chats_last_handle = ui.as_weak();
    let ui_audio_box = QBox::new(ui);

    ui.global::<Logic>().on_send_input_text(move |value| {
        let ui = ui_handle.unwrap();
        ui.global::<Logic>().invoke_clear_inst_tip();

        if value.trim().is_empty() {
            return;
        }

        let mut datas: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().collect();

        let uuid = Uuid::new_v4().to_string();

        datas.push(ChatItem {
            utext: value.clone(),
            btext: LOADING_STRING.into(),
            uuid: uuid.into(),
            timestamp: util::time::local_now("%Y-%m-%d %H:%M:%S").into(),
            utext_items: parse_chat_text(value.as_str()).into(),
            btext_items: parse_chat_text(LOADING_STRING).into(),
            ..Default::default()
        });

        let suuid = ui.global::<Store>().get_current_session_uuid();
        set_stop_chat(suuid.to_string(), false);

        let chat_datas = HistoryChat::from(&datas);

        ui.global::<Store>()
            .set_session_datas(Rc::new(VecModel::from(datas)).into());

        spawn(async move {
            if let Err(e) = send_text(ui_box, chat_datas).await {
                let etext = e.to_string();
                if let Err(err) = slint::invoke_from_event_loop(move || {
                    stream_text(
                        ui_box,
                        StreamTextItem {
                            etext: Some(etext),
                            ..Default::default()
                        },
                    );
                }) {
                    warn!("{:?}", err);
                }
            }
        });
    });

    ui.global::<Logic>().on_retry_send_input_text(move || {
        let ui = ui_retry_handle.unwrap();

        let rows = ui.global::<Store>().get_session_datas().row_count();
        if rows == 0 {
            return;
        }

        if let Some(item) = ui.global::<Store>().get_session_datas().row_data(rows - 1) {
            ui.global::<Logic>().invoke_remove_current_chats_last();
            ui.global::<Logic>().invoke_send_input_text(item.utext);
            ui.global::<Logic>()
                .invoke_show_message(tr("正在重试...").into(), "success".into());
        }
    });

    ui.global::<Logic>().on_delete_chat_item(move |uuid| {
        if uuid.trim().is_empty() {
            return;
        }

        let ui = ui_delete_handle.unwrap();
        let datas: Vec<ChatItem> = ui
            .global::<Store>()
            .get_session_datas()
            .iter()
            .filter(|x| x.uuid != uuid)
            .collect();

        ui.global::<Store>()
            .set_session_datas(Rc::new(VecModel::from(datas)).into());

        ui.global::<Logic>()
            .invoke_show_message((tr("删除成功") + "!").into(), "success".into());
    });

    ui.global::<Logic>().on_toggle_mark_chat_item(move |uuid| {
        if uuid.trim().is_empty() {
            return;
        }

        let ui = ui_mark_handle.unwrap();
        let datas: Vec<ChatItem> = ui
            .global::<Store>()
            .get_session_datas()
            .iter()
            .map(|x| {
                if x.uuid != uuid {
                    x
                } else {
                    let mut xc = x.clone();
                    xc.is_mark = !x.is_mark;
                    xc
                }
            })
            .collect();

        ui.global::<Store>()
            .set_session_datas(Rc::new(VecModel::from(datas)).into());
    });

    ui.global::<Logic>().on_stop_generate_text(move |suuid| {
        set_stop_chat(suuid.to_string(), true);
    });

    ui.global::<Logic>().on_text_to_speech(move |uuid, text| {
        let ui = ui_audio_handle.unwrap();
        let config = config::audio();

        if config.region.is_empty() || config.api_key.is_empty() {
            ui.global::<Logic>()
                .invoke_show_message((tr("请进行音频配置") + "!").into(), "info".into());
            return;
        }

        audio::azure::play(ui_audio_box, uuid.to_string() + ".mp3", text.to_string());
    });

    ui.global::<Logic>().on_remove_current_chats_first(move || {
        let ui = ui_remove_chats_first_handle.unwrap();

        let model: VecModel<_> = ui
            .global::<Store>()
            .get_session_datas()
            .iter()
            .collect::<Vec<ChatItem>>()
            .into();
        if model.row_count() > 0 {
            model.remove(0);
            ui.global::<Store>()
                .set_session_datas(Rc::new(model).into());
        }
    });

    ui.global::<Logic>().on_remove_current_chats_last(move || {
        let ui = ui_remove_chats_last_handle.unwrap();

        let model: VecModel<_> = ui
            .global::<Store>()
            .get_session_datas()
            .iter()
            .collect::<Vec<ChatItem>>()
            .into();
        if model.row_count() > 0 {
            model.remove(model.row_count() - 1);
            ui.global::<Store>()
                .set_session_datas(Rc::new(model).into());
        }
    });
}

pub fn parse_chat_text(data: &str) -> Rc<VecModel<CodeTextItem>> {
    let items = VecModel::default();
    let mut cur_item = CodeTextItem::default();
    let mut in_code_block = false;

    // Only support code format likes this: \n```some code```\n
    for line in data.lines() {
        if line.trim().starts_with("```") {
            if in_code_block {
                in_code_block = false;
                if !cur_item.text.is_empty() {
                    cur_item.text = cur_item.text.trim_end().into();
                    items.push(cur_item.clone());
                }
                cur_item = CodeTextItem::default();
            } else {
                in_code_block = true;
                if !cur_item.text.is_empty() {
                    cur_item.text = cur_item.text.trim_end().into();
                    items.push(cur_item.clone());
                }
                cur_item = CodeTextItem::default();
            }
            continue;
        } else {
            if in_code_block && cur_item.text_type.is_empty() {
                cur_item.text_type = "code".into();
            }

            if !in_code_block  {
                if line.trim_start().starts_with("- ") {
                    if !cur_item.text.is_empty() {
                        cur_item.text = cur_item.text.trim_end().into();
                        items.push(cur_item.clone());
                        cur_item = CodeTextItem::default();
                    }
                    cur_item.text_type = "list-item".into();
                } else if cur_item.text_type.is_empty() {
                    cur_item.text_type = "plain".into();
                }
            }

            if cur_item.text_type == "list-item" {
                cur_item.text = line
                    .replacen("-", "⚫", 1)
                    .trim_end()
                    .into();
                items.push(cur_item.clone());
                cur_item = CodeTextItem::default();
            } else {
                cur_item.text.push_str(line);
                cur_item.text.push_str("\n");
            }
        }
    }

    if !cur_item.text.is_empty() {
        cur_item.text = cur_item.text.trim_end().into();
        items.push(cur_item.clone());
    }

    Rc::new(items)
}
