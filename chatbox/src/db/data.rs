use crate::slint_generatedAppWindow::{ChatItem, ChatSession};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionConfig {
    pub name: String,
    pub system_prompt: String,
    pub api_model: String,

    #[serde(default)]
    pub shortcut_instruction: String,

    pub use_history: bool,
    pub is_mark: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionChat {
    pub uuid: String,
    pub utext: String,
    pub btext: String,

    #[serde(default)]
    pub timestamp: String,

    pub is_mark: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionChats {
    pub chats: Vec<SessionChat>,
}

impl From<&ChatSession> for SessionConfig {
    fn from(cs: &ChatSession) -> Self {
        SessionConfig {
            name: cs.name.clone().into(),
            system_prompt: cs.system_prompt.clone().into(),
            api_model: cs.api_model.clone().into(),
            shortcut_instruction: cs.shortcut_instruction.clone().into(),
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
                    timestamp: item.timestamp.clone().into(),
                    is_mark: item.is_mark,
                })
                .collect(),
        }
    }
}
