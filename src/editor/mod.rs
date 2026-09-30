use std::collections::BTreeMap;

use crate::editor::document::Document;

mod document;

pub struct Editor {
    pub buffers: BTreeMap<usize, Document>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffers: BTreeMap::new(),
        }
    }
}
