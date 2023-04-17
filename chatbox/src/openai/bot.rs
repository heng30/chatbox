use super::data;
use crate::logic::StreamTextItem;
use log::{debug, warn};
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE};
use reqwest::Client;
use std::time::Duration;
use tokio_stream::StreamExt;
use crate::config::openai as openai_config;

fn headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(ACCEPT, "text/event-stream".parse().unwrap());

    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers
}

pub async fn generate_text(
    prompt: String,
    _uuid: String,
    cb: impl Fn(StreamTextItem),
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let config = openai_config();

    let request_body = data::request::ChatCompletion {
        messages: vec![data::request::Message {
            role: "user".to_string(),
            content: prompt,
        }],

        model: config.chat.model,
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
        .timeout(Duration::from_secs(30))
        .send()
        .await?
        .bytes_stream();

    loop {
        match tokio::time::timeout(Duration::from_secs(15), stream.next()).await {
            Ok(Some(Ok(chunk))) => {
                let body = String::from_utf8_lossy(&chunk);

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
            _ => break,
        }
    }

    Ok(())
}
