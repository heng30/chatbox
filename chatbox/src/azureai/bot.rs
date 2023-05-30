use super::data;
use crate::chat;
use crate::config::azureai as azureai_config;
use crate::logic::StreamTextItem;
use crate::util::http;
use log::{debug, warn};
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CACHE_CONTROL, CONTENT_TYPE};
use std::time::Duration;
use tokio_stream::StreamExt;

fn headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        "api-key",
        api_key.to_string().parse().unwrap(),
    );
    headers.insert(ACCEPT, "text/event-stream".parse().unwrap());

    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers
}

pub async fn generate_text(
    chat: data::request::AzureAIChat,
    api_model: String,
    uuid: String,
    cb: impl Fn(StreamTextItem),
) -> Result<(), Box<dyn std::error::Error>> {
    let client = http::client(http::ClientType::Azure)?;
    let config = azureai_config();

    let request_body = data::request::ChatCompletion {
        messages: chat.message,
        model: api_model,
        max_tokens: config.chat.max_tokens,
        temperature: config.chat.temperature,
        frequency_penalty: config.chat.frequency_penalty,
        presence_penalty: config.chat.presence_penalty,
        stream: true,
    };

    let mut stream = client
        .post(config.chat.url)
        .headers(headers(&config.api_key))
        .json(&request_body)
        .timeout(Duration::from_secs(config.request_timeout))
        .send()
        .await?
        .bytes_stream();

    if chat::is_stop_chat(&uuid) {
        return Ok(());
    }

    loop {
        match tokio::time::timeout(Duration::from_secs(config.stream_timeout), stream.next()).await
        {
            Ok(Some(Ok(chunk))) => {
                if chat::is_stop_chat(&uuid) {
                    return Ok(());
                }

                let body = String::from_utf8_lossy(&chunk);

                if let Ok(err) = serde_json::from_str::<data::response::Error>(&body) {
                    if let Some(estr) = err.error.get("message") {
                        cb(StreamTextItem {
                            etext: Some(estr.clone()),
                            ..Default::default()
                        });
                        debug!("{}", estr);
                    }
                    break;
                }

                if body.starts_with("data: [DONE]") {
                    break;
                }

                let lines: Vec<_> = body.split("\n\n").collect();

                for line in lines.into_iter() {
                    if !line.starts_with("data:") {
                        continue;
                    }

                    match serde_json::from_str::<data::response::ChatCompletionChunk>(&line[5..]) {
                        Ok(chunk) => {
                            let choice = &chunk.choices[0];
                            if choice.finish_reason.is_some() {
                                println!();
                                debug!("finish_reason: {}", choice.finish_reason.as_ref().unwrap());
                                break;
                            }

                            if choice.delta.contains_key("content") {
                                cb(StreamTextItem {
                                    text: Some(choice.delta["content"].clone()),
                                    ..Default::default()
                                });
                                print!("{}", choice.delta["content"]);
                            } else if choice.delta.contains_key("role") {
                                debug!("role: {}", choice.delta["role"]);
                                continue;
                            }
                        }
                        Err(e) => {
                            cb(StreamTextItem {
                                etext: Some(e.to_string()),
                                ..Default::default()
                            });
                            debug!("{}", &line);
                            debug!("{}", e);
                            break;
                        }
                    }
                }
            }
            Ok(None) => {
                println!();
                break;
            }
            Err(e) => {
                warn!("{}", e);
                break;
            }
            Ok(Some(Err(e))) => {
                warn!("{}", e);
                break;
            }
        }
    }

    Ok(())
}
