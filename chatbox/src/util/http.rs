use crate::config;
use reqwest::{Client, Proxy, Result};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientType {
    Local = -1,
    OpenAI = 0,
    Azure = 1,
}

pub fn client(cln_type: ClientType) -> Result<Client> {
    let conf = config::socks5();
    Ok(if conf.enabled {
        if (cln_type == ClientType::OpenAI && conf.openai)
            || (cln_type == ClientType::Azure && conf.azure)
        {
            let proxy = Proxy::all(format!("socks5://{}:{}", conf.url, conf.port))?;
            Client::builder().proxy(proxy).build()?
        } else {
            Client::new()
        }
    } else {
        Client::new()
    })
}
