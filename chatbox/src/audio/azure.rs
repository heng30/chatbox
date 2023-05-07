use super::data::{AzureTextItem, TextType};
use crate::config;
use bytes::Bytes;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::io::{Cursor, Read, Seek};
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;

pub fn play(audio_file: String, text: String) {
    let audio_filepath = config::audio_path() + "/" + &audio_file;

    spawn(async move {
        let path = Path::new(&audio_filepath);

        if let Err(e) = if path.exists() {
            play_audio_local(&audio_filepath)
        } else {
            let text = make_text(&text);
            if text.is_empty() {
                return;
            }

            text_to_speech(&text, &audio_filepath).await
        } {
            warn!("{:?}", e);
        };
    });
}

fn play_audio_local(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    play_audio(BufReader::new(file))
}

fn play_audio_memory(source: &Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(source.to_vec());
    play_audio(cursor)
}

fn play_audio<R>(source: R) -> Result<(), Box<dyn std::error::Error>>
where
    R: Read + Seek + Send + Sync + 'static,
{
    let source = Decoder::new(source)?;

    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    sink.append(source);
    sink.play();
    sink.sleep_until_end();
    Ok(())
}

async fn text_to_speech(text: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let audio_config = config::audio();

    let url = format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
        audio_config.region
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "Ocp-Apim-Subscription-Key",
        HeaderValue::from_str(&audio_config.api_key)?,
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/ssml+xml"),
    );
    headers.insert(
        "X-Microsoft-OutputFormat",
        HeaderValue::from_static("audio-16khz-128kbitrate-mono-mp3"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("reqwest"));

    let body = format!(
        "<speak version='1.0' xml:lang='en-US'>
            <voice name='en-US-JennyMultilingualNeural'>
                {}
            </voice>
        </speak>",
        text
    );

    let response = reqwest::ClientBuilder::new()
        .build()?
        .post(&url)
        .headers(headers)
        .body(body.to_owned())
        .send()
        .await?;

    let response_body = response.bytes().await?;

    play_audio_memory(&response_body)?;

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)
        .await?;

    output_file.write_all(&response_body).await?;

    Ok(())
}

fn split_text(text: &str) -> Vec<AzureTextItem> {
    let mut result: Vec<AzureTextItem> = vec![];
    let mut last_text_type = None;

    for c in text.chars() {
        let text_type = if c.is_ascii() {
            TextType::EnUs
        } else {
            TextType::ZhCn
        };

        if last_text_type == Some(text_type) {
            result.last_mut().unwrap().text += &c.to_string();
        } else {
            result.push(AzureTextItem {
                text_type,
                text: c.to_string(),
            });
        }

        last_text_type = Some(text_type);
    }

    result
        .into_iter()
        .filter(|item| {
            !item
                .text
                .trim()
                .trim_matches(|c: char| {
                    c.is_ascii_punctuation() || c.is_control() || c.is_whitespace()
                })
                .trim()
                .is_empty()
        })
        .map(|item| AzureTextItem {
            text_type: item.text_type,
            text: item.text.trim().to_string()
                + if item.text_type == TextType::EnUs {
                    "."
                } else {
                    "。"
                },
        })
        .collect()
}

fn make_text(text: &str) -> String {
    let mut otext = String::default();
    let items = split_text(text);

    for item in items.into_iter() {
        otext += {
            if item.text_type == TextType::EnUs {
                format!("<lang xml:lang='en-US'> {} </lang>", item.text)
            } else if item.text_type == TextType::ZhCn {
                format!("<lang xml:lang='zh-CN'> {} </lang>", item.text)
            } else {
                String::default()
            }
        }
        .as_str();
    }

    otext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_text() {
        let text = "Hello, 世界！One World! 你好吗？";
        let expected_result = vec![
            AzureTextItem {
                text_type: TextType::EnUs,
                text: String::from("Hello,"),
            },
            AzureTextItem {
                text_type: TextType::ZhCn,
                text: String::from("世界！"),
            },
            AzureTextItem {
                text_type: TextType::EnUs,
                text: String::from("One World!"),
            },
            AzureTextItem {
                text_type: TextType::ZhCn,
                text: String::from("你好吗？"),
            },
        ];
        assert_eq!(split_text(text), expected_result);
    }
}
