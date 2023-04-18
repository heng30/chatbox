use super::data::{HistoryChat, StopChat, StreamTextItem};
use crate::config::openai as openai_config;
use crate::openai;
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, Logic, Store};
use crate::util::qbox::QBox;
#[allow(unused_imports)]
use log::{debug, warn};
use slint::{ComponentHandle, Model, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use tokio::task::spawn;
use uuid::Uuid;

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

    let prompt = if !chats.items.is_empty() {
        let history = openai::OpenAIHistoryChat::from(chats).to_json()?;
        format!(
            "{}{}{}",
            history,
            openai_config().chat.history_chat_explain,
            question
        )
    } else {
        question
    };

    debug!("{}", prompt);

    openai::generate_text(prompt, item.uuid, move |sitem| {
        if let Err(e) = slint::invoke_from_event_loop(move || {
            stream_text(ui_box, sitem);
        }) {
            warn!("{:?}", e);
        }
    })
    .await
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
        ui.global::<Store>().get_session_datas().set_row_data(
            current_row,
            ChatItem {
                btext: if item.btext == LOADING_STRING {
                    text.trim_start().into()
                } else {
                    item.btext + &text
                },
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
            .invoke_show_message("删除成功！".into(), "success".into());
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
}
