use crate::openai;
use crate::qbox::QBox;
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, Logic, Store};
use log::debug;
use slint::{ComponentHandle, Model, VecModel};
use std::env;
use std::rc::Rc;
use tokio::task::spawn;

#[derive(Default, Clone, Debug)]
pub struct StreamTextItem {
    pub text: Option<String>,
    pub etext: Option<String>,
}

const LOADING_STRING: &str = "loading...";

fn stream_text(ui_box: QBox<AppWindow>, sitem: StreamTextItem) {
    let ui = ui_box.borrow();
    let current_row = ui.global::<Store>().get_session_datas().row_count() - 1;

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

pub fn chat_with_bot(ui: &AppWindow) {
    let ui_box = QBox::new(ui);
    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_send_input_text(move |value| {
        if value.trim().is_empty() {
            return;
        }

        let ui = ui_handle.unwrap();
        let mut datas: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().collect();

        let prompt = format!("{}", value);
        debug!("{}", prompt);

        datas.push(ChatItem {
            utext: value,
            btext: LOADING_STRING.into(),
            ..Default::default()
        });

        ui.global::<Store>()
            .set_session_datas(Rc::new(VecModel::from(datas)).into());

        let cb = qmetaobject::queued_callback(move |sitem: StreamTextItem| {
            stream_text(ui_box, sitem);
        });

        spawn(async move {
            let api_key = env::var("OPENAI_API_KEY").unwrap();
            if let Err(e) = openai::generate_text(prompt, api_key, move |sitem| {
                cb(sitem);
            })
            .await
            {
                stream_text(ui_box, StreamTextItem {
                    etext: Some(e.to_string()),
                    ..Default::default()
                });
            }
        });
    });
}
