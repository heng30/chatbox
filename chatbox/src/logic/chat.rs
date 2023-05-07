use super::data::{HistoryChat, StopChat, StreamTextItem};
use crate::audio;
use crate::openai;
use crate::session;
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, CodeTextItem, Logic, Store};
use crate::util::qbox::QBox;
use crate::util::translator::tr;
#[allow(unused_imports)]
use log::{debug, warn};
use slint::{ComponentHandle, Model, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use tokio::task::spawn;
use uuid::Uuid;
use crate::config;

const LOADING_STRING: &str = "Thinking...";

lazy_static! {
    static ref STOP_CHAT: Mutex<RefCell<StopChat>> = Mutex::new(RefCell::new(StopChat::default()));
}

pub fn set_stop_chat(uuid: Option<String>, is_stop: bool) {
    let item = STOP_CHAT.lock().unwrap();
    let mut item = item.borrow_mut();
    if let Some(uid) = uuid {
        item.current_chat_item_uuid = uid;
    }
    item.is_stop = is_stop;
}

pub fn is_stop_chat(uuid: &str) -> bool {
    let item = STOP_CHAT.lock().unwrap();
    let item = item.borrow();

    // debug!("{} - {} - {}", item.is_stop, item.current_chat_item_uuid, uuid);
    item.is_stop || item.current_chat_item_uuid != uuid
}

async fn send_text(
    ui_box: QBox<AppWindow>,
    mut chats: HistoryChat,
) -> Result<(), Box<dyn std::error::Error>> {
    let item = chats.items.pop().unwrap();
    let question = item.utext;

    let (system_prompt, api_model, use_history) =
        session::current_session_config(ui_box.borrow().as_weak());

    if api_model.contains("chat-3.5-turbo") {
        let openai_chat = openai::OpenAIChat::make(
            system_prompt,
            question,
            if use_history {
                chats
            } else {
                HistoryChat::default()
            },
        );

        debug!("{:?}", openai_chat);

        return openai::generate_text(openai_chat, item.uuid, move |sitem| {
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
    let rows = ui.global::<Store>().get_session_datas().row_count();
    if rows == 0 {
        return;
    }

    let current_row = rows - 1;

    let text = match sitem.etext {
        Some(etext) => format!("\n\n{}", etext),
        _ => match sitem.text {
            Some(txt) => txt,
            _ => "".to_string(),
        },
    };

    if let Some(item) = ui
        .global::<Store>()
        .get_session_datas()
        .row_data(current_row)
    {
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
    }
}

pub fn init(ui: &AppWindow) {
    let ui_box = QBox::new(ui);
    let ui_handle = ui.as_weak();
    let ui_delete_handle = ui.as_weak();
    let ui_mark_handle = ui.as_weak();
    let ui_audio_handle = ui.as_weak();
    let ui_audio_box = QBox::new(ui);

    ui.global::<Logic>().on_send_input_text(move |value| {
        if value.trim().is_empty() {
            return;
        }

        let ui = ui_handle.unwrap();
        let mut datas: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().collect();

        let uuid = Uuid::new_v4().to_string();

        datas.push(ChatItem {
            utext: value,
            btext: LOADING_STRING.into(),
            uuid: uuid.as_str().into(),
            btext_items: parse_chat_text(LOADING_STRING).into(),
            ..Default::default()
        });

        set_stop_chat(Some(uuid), false);

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

    ui.global::<Logic>().on_stop_generate_text(move || {
        set_stop_chat(None, true);
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
}

pub fn parse_chat_text(data: &str) -> Rc<VecModel<CodeTextItem>> {
    let items = VecModel::default();
    let mut cur_item = CodeTextItem::default();
    let mut in_code_block = false;

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

            if !in_code_block && cur_item.text_type.is_empty() {
                cur_item.text_type = "plain".into();
            }

            cur_item.text.push_str(line);
            cur_item.text.push_str("\n");
        }
    }

    if !cur_item.text.is_empty() {
        cur_item.text = cur_item.text.trim_end().into();
        items.push(cur_item.clone());
    }

    Rc::new(items)
}
