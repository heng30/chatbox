use reqwest::header::{ACCEPT, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE, USER_AGENT};
use reqwest::Client;
use reqwest::header::HeaderMap;
use tokio_stream::StreamExt;

use std::time::Duration;
use super::data;
// use log::debug;

fn headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(ACCEPT, "text/event-stream".parse().unwrap());

    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36".parse().unwrap());
    headers
}

pub async fn generate_text(
    prompt: String,
    api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions".to_string();

    let request_body = data::request::ChatCompletion {
        messages: vec![data::request::Message {
            role: "user".to_string(),
            content: prompt,
        }],

        model: "gpt-3.5-turbo".to_string(),
        max_tokens: 100,
        temperature: 0.8,
        frequency_penalty: 0.5,
        presence_penalty: 0.0,
        stop: vec![".".to_string()],
        n: 1,
        stream: true,
    };


    let mut stream  = client
        .post(url)
        .headers(headers(&api_key))
        .json(&request_body)
        .send()
        .await?
        .bytes_stream();

    loop {
        match tokio::time::timeout(Duration::from_secs(30), stream.next()).await {
            Ok(Some(Ok(chunk))) => {
                let line = String::from_utf8_lossy(&chunk);
                println!("Received event: {}", line);
                if line.starts_with("data: [DONE]") {
                    break;
                }
            }
            Ok(None) => {
                println!("Stream ended");
                break;
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e);
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
