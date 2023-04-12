use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE};
use reqwest::Client;
use tokio_stream::StreamExt;

use super::data;
use log::{debug, warn};
use std::time::Duration;

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
    api_key: String,
    cb: impl Fn(String)
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions".to_string();

    let request_body = data::request::ChatCompletion {
        messages: vec![data::request::Message {
            role: "user".to_string(),
            content: prompt,
        }],

        model: "gpt-3.5-turbo".to_string(),
        max_tokens: 1024,
        temperature: 0.8,
        frequency_penalty: 0.5,
        presence_penalty: 0.0,
        stream: true,
    };

    let mut stream = client
        .post(url)
        .headers(headers(&api_key))
        .json(&request_body)
        .send()
        .await?
        .bytes_stream();

    loop {
        match tokio::time::timeout(Duration::from_secs(30), stream.next()).await {
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
                                cb(choice.delta["content"].clone());
                                print!("{}", choice.delta["content"]);
                            } else if choice.delta.contains_key("role") {
                                debug!("role: {}", choice.delta["role"]);
                                continue;
                            }
                        }
                        Err(e) => {
                            debug!("{}", e);
                            break;
                        }
                    }
                }
            }
            Ok(None) => {
                println!("");
                break;
            }
            Err(e) => {
                warn!("{}", e);
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
