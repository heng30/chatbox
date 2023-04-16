use super::data::{self, Config};
use crate::CResult;
use log::debug;
use platform_dirs::AppDirs;
use std::cell::RefCell;
use std::sync::Mutex;
use std::{env, fs};

lazy_static! {
    pub static ref CONFIG: Mutex<RefCell<Config>> = Mutex::new(RefCell::new(Config::default()));
}

pub fn init() {
    if let Err(e) = CONFIG.lock().unwrap().borrow_mut().init() {
        panic!("{:?}", e);
    }
}

pub fn openai() -> data::OpenAi {
    CONFIG.lock().unwrap().borrow_mut().openai.clone()
}

impl Config {
    pub fn init(&mut self) -> CResult {
        let app_dirs = AppDirs::new(Some("chatbox"), true).unwrap();
        Self::init_app_dir(&app_dirs)?;
        self.init_config(&app_dirs)?;
        self.load()?;
        debug!("{:?}", self);
        Ok(())
    }

    fn init_app_dir(app_dirs: &AppDirs) -> CResult {
        fs::create_dir_all(&app_dirs.data_dir)?;
        fs::create_dir_all(&app_dirs.config_dir)?;
        Ok(())
    }

    fn init_config(&mut self, app_dirs: &AppDirs) -> CResult {
        self.config_path = app_dirs
            .config_dir
            .join("chatbox.conf")
            .to_str()
            .unwrap()
            .to_string();

        self.db_path = app_dirs
            .data_dir
            .join("chatbox.db")
            .to_str()
            .unwrap()
            .to_string();

        let mut dir = env::current_exe()?;
        dir.pop();
        self.working_dir = dir.to_str().unwrap().to_string();
        Ok(())
    }

    fn load(&mut self) -> CResult {
        match fs::read_to_string(&self.config_path) {
            Ok(text) => match serde_json::from_str::<Config>(&text) {
                Ok(c) => {
                    self.openai = c.openai;
                    Ok(())
                }
                Err(e) => Err(anyhow::anyhow!("{}", e.to_string()).into()),
            },

            Err(_) => match serde_json::to_string_pretty(self) {
                Ok(text) => Ok(fs::write(&self.config_path, text)?),
                Err(e) => Err(anyhow::anyhow!("{}", e.to_string()).into()),
            },
        }
    }

    #[allow(dead_code)]
    pub fn save(&self) -> CResult {
        match serde_json::to_string_pretty(self) {
            Ok(text) => Ok(fs::write(&self.config_path, text)?),
            Err(e) => Err(anyhow::anyhow!("{}", e.to_string()).into()),
        }
    }
}
