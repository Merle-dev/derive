use std::{collections::BTreeMap, fs::File, io::Read};

use crate::{
    components::bottombar::BottomBarTask,
    editor::document::Document,
    keys::{KeyMap, Mode},
};

mod document;
mod mode;

pub struct Editor {
    pub buffers: BTreeMap<usize, Document>,
    pub key_map: KeyMap,
    pub bartask: BottomBarTask,
    pub mode: Mode,
    pub quit: bool,
}

impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let mut file = File::open("keys.toml")?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        Ok(Self {
            buffers: BTreeMap::new(),
            key_map: KeyMap::load(text)?,
            bartask: BottomBarTask::Display,
            mode: Mode::Normal,
            quit: false,
        })
    }
}
