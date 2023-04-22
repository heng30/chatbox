use crate::slint_generatedAppWindow::{ChatItem, ChatSession};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionConfig {
    name: String,
    system_prompt: String,
    api_model: String,
    use_history: bool,
    is_mark: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionChat {
    uuid: String,
    utext: String,
    btext: String,
    is_mark: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionChats {
    chats: Vec<SessionChat>,
}

impl From<&ChatSession> for SessionConfig {
    fn from(cs: &ChatSession) -> Self {
        SessionConfig {
            name: cs.name.clone().into(),
            system_prompt: cs.system_prompt.clone().into(),
            api_model: cs.api_model.clone().into(),
            use_history: cs.use_history,
            is_mark: cs.is_mark,
        }
    }
}

impl From<&Vec<ChatItem>> for SessionChats {
    fn from(items: &Vec<ChatItem>) -> Self {
        SessionChats {
            chats: items
                .iter()
                .map(|item| SessionChat {
                    uuid: item.uuid.clone().into(),
                    utext: item.utext.clone().into(),
                    btext: item.btext.clone().into(),
                    is_mark: item.is_mark,
                })
                .collect(),
        }
    }
}
