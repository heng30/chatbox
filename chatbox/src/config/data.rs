#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: String,

    #[serde(skip)]
    pub config_path: String,

    #[serde(skip)]
    pub db_path: String,

    pub socks5: Socks5,

    pub openai: OpenAi,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Socks5 {
    pub enable: bool,
    pub url: String,
    pub port: u16,
}

impl Default for Socks5 {
    fn default() -> Self {
        Self {
            enable: false,
            url: "127.0.0.1".to_string(),
            port: 1080,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAi {
    pub request_timeout: u64,
    pub stream_timeout: u64,
    pub api_key: String,
    pub chat: OpenAiChat,
}

impl Default for OpenAi {
    fn default() -> Self {
        Self {
            request_timeout: 30,
            stream_timeout: 15,
            api_key: String::default(),
            chat: OpenAiChat::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiChat {
    pub url: String,
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

impl Default for OpenAiChat {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 1024,
            temperature: 0.8,
            frequency_penalty: 0.5,
            presence_penalty: 0.0,
        }
    }
}
