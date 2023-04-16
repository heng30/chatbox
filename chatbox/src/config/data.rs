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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAi {
    pub url: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

impl Default for OpenAi {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            api_key: String::default(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 1024,
            temperature: 0.8,
            frequency_penalty: 0.5,
            presence_penalty: 0.0,
        }
    }
}
