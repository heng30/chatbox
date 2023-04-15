use crate::slint_generatedAppWindow::{AppWindow, ChatSession, Logic, Store};
use slint::{ComponentHandle, Model, VecModel};
use std::rc::Rc;
use uuid::Uuid;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_handle_new_session(move |value| {
        let ui = ui_handle.unwrap();
        let mut sessions: Vec<ChatSession> =
            ui.global::<Store>().get_chat_sessions().iter().collect();

        sessions.push(ChatSession {
            name: value,
            uuid: Uuid::new_v4().to_string().into(),
        });

        let sessions_model = Rc::new(VecModel::from(sessions));
        ui.global::<Store>()
            .set_chat_sessions(sessions_model.into());
    });
}
