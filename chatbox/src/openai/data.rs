pub mod request {
    use crate::logic::HistoryChat;
    use std::fmt::Debug;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ChatCompletion {
        pub messages: Vec<Message>,
        pub model: String,
        pub max_tokens: i32,
        pub temperature: f32,
        pub frequency_penalty: f32,
        pub presence_penalty: f32,
        pub stream: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct OpenAIChat {
        pub message: Vec<Message>,
    }

    impl OpenAIChat {
        pub fn make(system_prompt: String, question: String, chats: HistoryChat) -> OpenAIChat {
            let mut items = vec![];
            items.push(Message {
                role: "system".to_string(),
                content: system_prompt,
            });

            for item in chats.items.into_iter() {
                items.push(Message {
                    role: "user".to_string(),
                    content: item.utext,
                });
                items.push(Message {
                    role: "assistant".to_string(),
                    content: item.btext,
                })
            }

            items.push(Message {
                role: "user".to_string(),
                content: question,
            });

            OpenAIChat { message: items }
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

    #[derive(Serialize, Deserialize)]
    pub struct Error {
        pub error: HashMap<String, String>,
    }
}
