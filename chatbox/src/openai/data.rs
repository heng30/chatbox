pub mod request {
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
