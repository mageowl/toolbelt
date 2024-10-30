use std::{env, fs, path::PathBuf};

use serde::Deserialize;

use crate::style::{Style, Styled};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub prompt: Text,
    pub icon: Text,
    pub window_size: Option<(usize, usize)>,

    #[serde(flatten)]
    pub menu: MenuConfig,
}

impl Config {
    pub fn get_menu(name: String) -> Self {
        let mut menu_path =
            PathBuf::from(env::var_os("HOME").expect("failed to get home directory"));
        menu_path.push(".config/toolbelt/");
        menu_path.push(name + ".json");

        let file = fs::read_to_string(menu_path).expect("failed to open config file");
        serde_json::from_str(&file).expect("failed to parse config")
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MenuConfig {
    #[serde(rename_all = "camelCase")]
    List {
        entries: Vec<Entry>,
        #[serde(default)]
        selected_style: Style,
    },
    Prompt {
        #[serde(flatten)]
        action: Action,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Text {
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

    #[serde(flatten)]
    pub action: Action,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Exec(String),
    Command {
        cmd: String,
        args: Vec<String>,
        #[serde(default)]
        hold_output: bool,
    },
    OpenMenu(String),
}
