mod conf;
mod data;

pub use conf::{init, openai, path, socks5, ui, config, save};
pub use data::Config;
