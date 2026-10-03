use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use ratatui::layout::Position;
use ropey::Rope;

use crate::{
    components::bottombar::BottomBarTask,
    editor::document::Document,
    keys::{KeyMap, Mode},
};

mod document;
mod mode;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct DocumentId(usize);

pub struct Editor {
    pub buffers: BTreeMap<DocumentId, Document>,
    pub key_map: KeyMap,
    pub bartask: BottomBarTask,
    pub cursor: Option<Position>,
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
            cursor: None,
            quit: false,
        })
    }
    pub fn add_doc(&mut self, path: PathBuf) -> anyhow::Result<DocumentId> {
        let id = DocumentId(self.buffers.len());
        self.buffers.insert(
            id,
            Document {
                rope: Rope::from_reader(BufReader::new(File::open(path.clone())?))?,
                path,
            },
        );
        Ok(id)
    }
}
