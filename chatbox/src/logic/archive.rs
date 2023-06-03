use super::chat;
use crate::db::{self, data::SessionChats};
use crate::slint_generatedAppWindow::{AppWindow, ArchiveChatItem, ChatItem, Logic, Store};
use crate::util::translator::tr;
use log::warn;
use slint::{ComponentHandle, Model, SortModel, VecModel};
use std::rc::Rc;
use uuid::Uuid;

pub fn init(ui: &AppWindow) {
    let ui_save_handle = ui.as_weak();
    let ui_delete_session_archive_handle = ui.as_weak();
    let ui_delete_session_archives_handle = ui.as_weak();
    let ui_show_handle = ui.as_weak();
    let ui_show_list_handle = ui.as_weak();

    ui.global::<Logic>()
        .on_save_session_archive(move |suuid, name| {
            let ui = ui_save_handle.unwrap();
            let uuid = Uuid::new_v4().to_string();

            let mut datas: Vec<ArchiveChatItem> = ui
                .global::<Store>()
                .get_session_archive_datas()
                .iter()
                .collect();

            datas.push(ArchiveChatItem {
                name: name.as_str().into(),
                uuid: uuid.as_str().into(),
            });

            ui.global::<Store>()
                .set_session_archive_datas(Rc::new(VecModel::from(datas)).into());

            let chats: Vec<ChatItem> = ui.global::<Store>().get_session_datas().iter().collect();
            if chats.is_empty() {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}", tr("没有可归档的对话") + "！"),
                    "info".into(),
                );
            }

            match serde_json::to_string::<SessionChats>(&SessionChats::from(&chats)) {
                Ok(text) => {
                    if let Err(e) = match db::archive::is_table_exist(suuid.as_str()) {
                        Ok(true) => db::archive::insert(
                            suuid.as_str(),
                            uuid.as_str(),
                            name.as_str(),
                            text.as_str(),
                        ),
                        _ => match db::archive::new(suuid.as_str()) {
                            Ok(_) => db::archive::insert(
                                suuid.as_str(),
                                uuid.as_str(),
                                name.as_str(),
                                text.as_str(),
                            ),
                            Err(e) => Err(e),
                        },
                    } {
                        ui.global::<Logic>().invoke_show_message(
                            slint::format!("{}: {:?}", tr("保存失败") + "！" + &tr("原因"), e),
                            "warning".into(),
                        );
                    } else {
                        ui.global::<Logic>()
                            .invoke_show_message((tr("保存成功") + "!").into(), "success".into());
                    }
                }
                Err(e) => {
                    ui.global::<Logic>().invoke_show_message(
                        slint::format!("{}: {:?}", tr("保存失败") + "！" + &tr("原因"), e),
                        "warning".into(),
                    );
                }
            }
        });

    ui.global::<Logic>()
        .on_delete_session_archive(move |suuid, uuid| {
            let ui = ui_delete_session_archive_handle.unwrap();
            let datas: Vec<ArchiveChatItem> = ui
                .global::<Store>()
                .get_session_archive_datas()
                .iter()
                .filter(|item| item.uuid != uuid)
                .collect();

            ui.global::<Store>()
                .set_session_archive_datas(Rc::new(VecModel::from(datas)).into());

            match db::archive::delete(suuid.as_str(), uuid.as_str()) {
                Err(e) => ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("删除失败") + "!" + &tr("原因"), e),
                    "warning".into(),
                ),
                _ => ui
                    .global::<Logic>()
                    .invoke_show_message((tr("删除成功") + "!").into(), "success".into()),
            }
        });

    ui.global::<Logic>()
        .on_delete_session_archives(move |suuid| {
            let ui = ui_delete_session_archives_handle.unwrap();

            ui.global::<Store>()
                .set_session_archive_datas(Rc::new(VecModel::default()).into());

            match db::archive::drop_table(suuid.as_str()) {
                Err(e) => ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("删除失败") + "!" + &tr("原因"), e),
                    "warning".into(),
                ),
                _ => ui
                    .global::<Logic>()
                    .invoke_show_message((tr("删除成功") + "!").into(), "success".into()),
            }
        });

    ui.global::<Logic>()
        .on_show_session_archive(move |suuid, uuid| {
            let ui = ui_show_handle.unwrap();

            match db::archive::is_table_exist(suuid.as_str()) {
                Ok(false) => return,
                Err(e) => {
                    ui.global::<Logic>().invoke_show_message(
                        slint::format!("{}: {:?}", tr("获取归档文件失败") + "!" + &tr("原因"), e),
                        "warning".into(),
                    );
                    return;
                }
                _ => (),
            }

            match db::archive::select(suuid.as_str(), uuid.as_str()) {
                Ok(Some(item)) => {
                    let data = item.1;
                    if data.is_empty() {
                        return;
                    }

                    match serde_json::from_str::<SessionChats>(&data) {
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

                            ui.global::<Store>()
                                .set_session_datas(Rc::new(chat_items).into());
                        }

                        Err(e) => ui.global::<Logic>().invoke_show_message(
                            slint::format!(
                                "{}: {:?}",
                                tr("获取归档文件失败") + "!" + &tr("原因"),
                                e
                            ),
                            "warning".into(),
                        ),
                    }
                }
                Ok(_) => (),
                Err(e) => ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("获取归档文件失败") + "!" + &tr("原因"), e),
                    "warning".into(),
                ),
            }
        });

    ui.global::<Logic>()
        .on_show_session_archive_list(move |suuid| {
            let ui = ui_show_list_handle.unwrap();

            match db::archive::is_table_exist(suuid.as_str()) {
                Ok(true) => (),
                Err(e) => {
                    ui.global::<Store>()
                        .set_session_archive_datas(Rc::new(VecModel::default()).into());
                    warn!("{:?}", e);
                    return;
                }
                _ => {
                    ui.global::<Store>()
                        .set_session_archive_datas(Rc::new(VecModel::default()).into());
                    return;
                }
            }

            match db::archive::select_all(suuid.as_str()) {
                Ok(items) => {
                    let search_text = ui.global::<Store>().get_archive_search_text();
                    let aitems = VecModel::default();
                    for item in items.into_iter() {
                        if search_text.is_empty() || item.1.contains(search_text.as_str()) {
                            aitems.push(ArchiveChatItem {
                                uuid: item.0.into(),
                                name: item.1.into(),
                            });
                        }
                    }

                    let aitems = SortModel::new(aitems, |a, b| {
                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                    });

                    ui.global::<Store>()
                        .set_session_archive_datas(Rc::new(aitems).into());
                }
                Err(e) => ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}: {:?}", tr("获取归档文件失败") + "!" + &tr("原因"), e),
                    "warning".into(),
                ),
            }
        });
}
