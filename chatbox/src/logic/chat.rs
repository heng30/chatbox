use super::data::{HistoryChat, StreamTextItem};
use crate::openai;
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, Logic, Store};
use crate::util::qbox::QBox;

#[allow(unused_imports)]
use log::{debug, warn};
use slint::{ComponentHandle, Model, VecModel};
use std::env;
use std::rc::Rc;
use tokio::task::spawn;
use uuid::Uuid;

const LOADING_STRING: &str = "loading...";
const HISTORY_CHAT_EXPLAIN: &str = "\nThe above json fromat text is the previous conversations. If 'user' value is 'customer', the 'text' value is my previous question. if 'user' value is 'bot', the 'text' value is your reply. Parser the json format text And answer me the new question according the previous conversations. My new question: ";

async fn send_text(
    ui_box: QBox<AppWindow>,
    mut chats: HistoryChat,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();

    let question = chats.items.pop().unwrap().utext;

    let history;
    let prompt;

    if !chats.items.is_empty() {
        history = openai::OpenAIHistoryChat::from(chats).to_json()?;
        prompt = format!("{}{}{}", history, HISTORY_CHAT_EXPLAIN, question);
    } else {
        prompt = question;
    }

    debug!("{}", prompt);

    openai::generate_text(prompt, api_key, move |sitem| {
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
        return ;
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
    ui.global::<Logic>().on_send_input_text(move |value| {
        if value.trim().is_empty() {
            return;
        }

        let ui = ui_handle.unwrap();
        let mut datas: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().collect();

        datas.push(ChatItem {
            utext: value,
            btext: LOADING_STRING.into(),
            uuid: Uuid::new_v4().to_string().into(),
            ..Default::default()
        });

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
        let datas: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().filter(|x| x.uuid != uuid).collect();

        ui.global::<Store>()
            .set_session_datas(Rc::new(VecModel::from(datas)).into());

        ui.global::<Logic>()
            .invoke_show_message("删除成功！".into(), "success".into());
    });


}
