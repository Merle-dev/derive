use crate::{editor::Editor, keys::Mode};

impl Editor {
    pub fn into_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.key_map.set_mode(Mode::Normal);
    }
    pub fn into_insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.key_map.set_mode(Mode::Insert);
    }
    pub fn into_visual_mode(&mut self) {
        self.mode = Mode::Visual;
        self.key_map.set_mode(Mode::Visual);
    }
}
