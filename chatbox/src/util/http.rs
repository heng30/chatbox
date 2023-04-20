use crate::config;
use reqwest::{Client, Proxy, Result};

pub fn client() -> Result<Client> {
    let conf = config::socks5();
    Ok(if conf.enable {
        let proxy = Proxy::all(format!("socks5://{}:{}", conf.url, conf.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    })
}
