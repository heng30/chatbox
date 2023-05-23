pub mod request {
    pub use crate::openai::data::request::ChatCompletion;
    pub use crate::openai::data::request::OpenAIChat as AzureAIChat;
}

pub mod response {
    pub use crate::openai::data::response::ChatCompletionChunk;
    pub use crate::openai::data::response::ChunkChoice;
    pub use crate::openai::data::response::Error;
}
