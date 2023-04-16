pub mod request {
    use crate::logic::HistoryChat;
    use std::convert::From;

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletion {
        pub messages: Vec<Message>,
        pub model: String,
        pub max_tokens: i32,
        pub temperature: f32,
        pub frequency_penalty: f32,
        pub presence_penalty: f32,
        pub stream: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct OpenAIHistoryChatItem {
        pub id: String,
        pub text: String,
        pub user: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct OpenAIHistoryChat {
        pub utterances: Vec<OpenAIHistoryChatItem>,
    }

    impl From<HistoryChat> for OpenAIHistoryChat {
        fn from(chats: HistoryChat) -> Self {
            let mut items = vec![];
            for (i, item) in chats.items.into_iter().enumerate() {
                items.push(OpenAIHistoryChatItem {
                    id: format!("{}", i * 2 + 1),
                    text: item.utext,
                    user: "customer".to_string(),
                });
                items.push(OpenAIHistoryChatItem {
                    id: format!("{}", i * 2 + 2),
                    text: item.btext,
                    user: "bot".to_string(),
                })
            }

            OpenAIHistoryChat { utterances: items }
        }
    }

    impl OpenAIHistoryChat {
        pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
            match serde_json::to_string::<OpenAIHistoryChat>(self) {
                Ok(text) => Ok(text),
                Err(e) => Err(anyhow::anyhow!(
                    "OpenAIHistoryChat to json error: {}",
                    e.to_string()
                )
                .into()),
            }
        }
    }
}

pub mod response {
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletionChunk {
        pub id: String,
        pub object: String,
        pub created: i64,
        pub model: String,
        pub choices: Vec<ChunkChoice>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ChunkChoice {
        pub delta: HashMap<String, String>,
        pub index: usize,
        pub finish_reason: Option<String>,
    }
}
