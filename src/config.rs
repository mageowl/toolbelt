use std::{env, fs, path::PathBuf};

use serde::Deserialize;

use crate::style::Styled;

pub struct Config {
    pub prompt: Styled,
    pub icon: Styled,

    pub entries: Option<Vec<Entry>>,
}

impl From<DeserializedCfg> for Config {
    fn from(value: DeserializedCfg) -> Self {
        Self {
            prompt: value.prompt.into(),
            icon: value.icon.into(),
            entries: value.entries,
        }
    }
}

impl Config {
    pub fn get_menu(name: String) -> Self {
        let mut menu_path =
            PathBuf::from(env::var_os("HOME").expect("failed to get home directory"));
        menu_path.push(".config/toolbelt/");
        menu_path.push(name + ".json");
        dbg!(&menu_path);

        let file = fs::read_to_string(menu_path).expect("failed to open config file");
        let cfg: DeserializedCfg = serde_json::from_str(&file).expect("failed to parse config");
        cfg.into()
    }
}

#[derive(Deserialize)]
struct DeserializedCfg {
    prompt: Text,
    icon: Text,
    entries: Option<Vec<Entry>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Text {
    Unstyled(String),
    Styled(Styled),
}

impl From<Text> for Styled {
    fn from(value: Text) -> Self {
        match value {
            Text::Styled(s) => s,
            Text::Unstyled(s) => Self::from(s),
        }
    }
}

#[derive(Deserialize)]
pub struct Entry {
    pub name: String,
    pub icon: String,
    pub keywords: Option<String>,

    pub action: Action,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    Exec {},
    #[serde(rename = "sh")]
    Command {},
}
