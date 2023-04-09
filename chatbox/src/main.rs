// slint::include_modules!();

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use chrono::Local;
use env_logger::fmt::Color as LColor;

use std::env;
use std::io::Write;

mod openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    debug!("{}", "start...");

    let prompt = "Serve me as a writing and programming assistant.\nshow me code how to send a http request in rust".to_string();
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let response = openai::generate_text(prompt, api_key).await?;

    // let ui = AppWindow::new();

    // let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });

    // ui.run();

    debug!("{}", "exit...");

    Ok(())
}

fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut level_style = buf.style();
            match record.level() {
                log::Level::Warn | log::Level::Error => {
                    level_style.set_color(LColor::Red).set_bold(true)
                }
                _ => level_style.set_color(LColor::Blue).set_bold(true),
            };

            writeln!(
                buf,
                "[{} {} {} {}] {}",
                ts,
                level_style.value(record.level()),
                record
                    .file()
                    .unwrap_or("None")
                    .split('/')
                    .last()
                    .unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
