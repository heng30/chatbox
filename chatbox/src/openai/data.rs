pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletion {
        pub messages: Vec<Message>,
        pub model: String,
        pub max_tokens: i32,
        pub temperature: f32,
        pub frequency_penalty: f32,
        pub presence_penalty: f32,
        pub stop: Vec<String>,
        pub n: i32,
        pub stream: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletion {
        pub id: String,
        pub object: String,
        pub created: i64,
        pub choices: Vec<CompletionChoice>,
        pub usage: Usage,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CompletionChoice {
        pub index: usize,
        pub message: Message,
        pub finish_reason: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Usage {
        pub prompt_tokens: usize,
        pub completion_tokens: usize,
        pub total_tokens: usize,
    }

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
        pub delta: Delta,
        pub index: usize,
        pub finish_reason: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Delta {
        pub content: String,
    }
}
