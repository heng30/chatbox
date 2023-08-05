use crate::slint_generatedAppWindow::{AppWindow, InstTipItem, Logic, Store};
use slint::{ComponentHandle, Model, ModelRc, VecModel};

pub fn init(ui: &AppWindow) {
    let ui_select_handle = ui.as_weak();
    let ui_clear_handle = ui.as_weak();
    let ui_refresh_handle = ui.as_weak();

    ui.global::<Logic>().on_select_inst(move |inst| {
        let ui = ui_select_handle.unwrap();
        ui.set_question(inst);
    });

    ui.global::<Logic>().on_clear_inst_tip(move || {
        let ui = ui_clear_handle.unwrap();

        let mut inst_tip = ui.global::<Store>().get_inst_tip_setting();
        if inst_tip.items.row_count() > 0 {
            inst_tip.items = ModelRc::new(VecModel::default());
            ui.global::<Store>().set_inst_tip_setting(inst_tip);
        }
    });

    ui.global::<Logic>().on_refresh_inst(move |question| {
        let ui = ui_refresh_handle.unwrap();
        let question = question.trim_start();
        let items = VecModel::default();

        if !question.is_empty() && question.find(" ").is_none() {
            let cur_session_id = ui.global::<Store>().get_current_session_uuid();
            for session in ui.global::<Store>().get_chat_sessions().iter() {
                if session.shortcut_instruction.len() < question.len()
                    || cur_session_id == session.uuid
                {
                    continue;
                }

                if session.shortcut_instruction.starts_with(question) {
                    items.push(InstTipItem {
                        inst: session.shortcut_instruction,
                        session_name: session.name,
                        session_icon_index: session.icon_index,
                    });
                }
            }
        }

        let mut inst_tip = ui.global::<Store>().get_inst_tip_setting();
        if items.row_count() <= 0 && inst_tip.items.row_count() <= 0 {
            return;
        }

        inst_tip.items = ModelRc::new(items);
        ui.global::<Store>().set_inst_tip_setting(inst_tip);
    });
}
