use super::data::{AzureTextItem, TextType};
use super::play;
use crate::config;
use crate::slint_generatedAppWindow::{AppWindow, Logic};
use crate::util::qbox::QBox;
use crate::util::translator::tr;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use slint::ComponentHandle;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;

static IS_PLAYING: AtomicBool = AtomicBool::new(false);

pub fn play(ui_box: QBox<AppWindow>, audio_file: String, text: String) {
    if IS_PLAYING.load(Ordering::SeqCst) {
        ui_box
            .borrow()
            .global::<Logic>()
            .invoke_show_message(slint::format!("{}", tr("正在播放...")), "info".into());
        return;
    } else {
        IS_PLAYING.store(true, Ordering::SeqCst);
    }

    let audio_filepath = if !audio_file.is_empty() {
        config::audio_path() + "/" + &audio_file
    } else {
        String::default()
    };

    spawn(async move {
        let path = Path::new(&audio_filepath);

        if let Err(e) = if !audio_filepath.is_empty() && path.exists() {
            play::audio_local(&audio_filepath)
        } else if !text.is_empty() {
            let text = make_text(&text);
            text_to_speech(&text, &audio_filepath).await
        } else {
            Err(anyhow::anyhow!(tr("没有本地缓存或没有提供文本") + "!").into())
        } {
            let estr = e.to_string();
            if let Err(err) = slint::invoke_from_event_loop(move || {
                ui_box.borrow().global::<Logic>().invoke_show_message(
                    slint::format!("{}: {}", tr("播放音频失败") + "!" + &tr("原因"), estr),
                    "warning".into(),
                );
            }) {
                warn!("{:?}", err);
            }
        };

        IS_PLAYING.store(false, Ordering::SeqCst);
    });
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
        .timeout(Duration::from_secs(audio_config.t2s_max_request_duration))
        .headers(headers)
        .body(body.to_owned())
        .send()
        .await?;

    let response_body = response.bytes().await?;

    play::audio_memory(&response_body)?;

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)
        .await?;

    output_file.write_all(&response_body).await?;

    Ok(())
}

// TODO
pub async fn speech_to_text(
    input_path: &str,
    language: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::default())
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
            text: item
                .text
                .trim()
                .trim_matches(|c: char| {
                    c.is_ascii_punctuation() || c.is_control() || c.is_whitespace()
                })
                .trim()
                .to_string()
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
