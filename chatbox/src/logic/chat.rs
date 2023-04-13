use crate::openai;
use crate::qbox::QBox;
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, Logic, Store};
use log::debug;
use slint::{ComponentHandle, Model, VecModel};
use std::env;
use std::rc::Rc;
use tokio::task::spawn;

const LOADING_STRING: &str = "loading...";

fn stream_text(ui_box: QBox<AppWindow>, text: String) {
    let ui = ui_box.borrow();
    let current_row = ui.global::<Store>().get_session_datas().row_count() - 1;

    if let Some(item) = ui
        .global::<Store>()
        .get_session_datas()
        .row_data(current_row)
    {
        ui.global::<Store>().get_session_datas().set_row_data(
            current_row,
            ChatItem {
                btext: if item.btext == LOADING_STRING {
                    text.into()
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

        let prompt = format!(
            "{} My question: {}",
            ui.global::<Store>().get_chat_current_prompt(),
            value,
        );

        debug!("{}", prompt);

        datas.push(ChatItem {
            utext: value,
            btext: LOADING_STRING.into(),
            ..Default::default()
        });


        ui.global::<Store>()
            .set_session_datas(Rc::new(VecModel::from(datas)).into());

        let cb = qmetaobject::queued_callback(move |text: String| {
            stream_text(ui_box, text);
        });

        spawn(async move {
            let api_key = env::var("OPENAI_API_KEY").unwrap();
            let _ = openai::generate_text(prompt, api_key, move |text| {
                cb(text);
            })
            .await;
        });
    });
}
