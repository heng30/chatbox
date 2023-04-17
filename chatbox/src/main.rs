slint::include_modules!();

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use chrono::Local;
use env_logger::fmt::Color as LColor;
use log::debug;
use std::env;
use std::io::Write;

mod logic;
mod openai;
mod util;
mod config;

use logic::{chat, clipboard, message, session, ok_cancel_dialog };

pub type CResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> CResult {
    init_logger();
    debug!("{}", "start...");

    config::init();

    let ui = AppWindow::new().unwrap();
    clipboard::init(&ui);
    message::init(&ui);
    session::init(&ui);
    chat::init(&ui);

    ok_cancel_dialog::init(&ui);
    ui.run().unwrap();

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
