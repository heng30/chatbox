
const OPENAI_HISTORY_CHAT_EXPLAIN: &str = "\nThe above json fromat text is the previous conversations. If 'user' value is 'customer', the 'text' value is my previous question. if 'user' value is 'bot', the 'text' value is your reply. Retrieve the necessary information from the previous conversation and use it to answer the new question. But if the new question is not related to the previous conversation. Please ignore the previous conversation and answer my new question: ";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: String,

    #[serde(skip)]
    pub config_path: String,

    #[serde(skip)]
    pub db_path: String,

    pub openai: OpenAi,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OpenAi {
    pub api_key: String,
    pub chat: OpenAiChat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiChat {
    pub url: String,
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,

    pub history_chat_explain: String,
}

impl Default for OpenAiChat {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 4096,
            temperature: 0.8,
            frequency_penalty: 0.5,
            presence_penalty: 0.0,

            history_chat_explain: OPENAI_HISTORY_CHAT_EXPLAIN.to_string(),
        }
    }
}
