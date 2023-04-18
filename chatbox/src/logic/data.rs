use crate::slint_generatedAppWindow::ChatItem;
use std::convert::From;

#[derive(Default, Clone, Debug)]
pub struct StreamTextItem {
    pub text: Option<String>,
    pub etext: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct HistoryChatItem {
    pub utext: String,
    pub btext: String,
    pub uuid: String,
}

#[derive(Default, Clone, Debug)]
pub struct HistoryChat {
    pub items: Vec<HistoryChatItem>,
}

impl From<&Vec<ChatItem>> for HistoryChat {
    fn from(item: &Vec<ChatItem>) -> Self {
        HistoryChat {
            items: item
                .iter()
                .map(|x| HistoryChatItem {
                    utext: x.utext.to_string(),
                    btext: x.btext.to_string(),
                    uuid: x.uuid.to_string(),
                })
                .collect(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct StopChat {
    pub current_chat_item_uuid: String,
    pub is_stop: bool,
}
