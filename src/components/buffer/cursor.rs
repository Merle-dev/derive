use ropey::Rope;

use crate::components::buffer::TextBuffer;

impl TextBuffer {
    pub fn cursor_up(&mut self, _rope: &Rope) {
        self.true_cursor.y = self.true_cursor.y.saturating_sub(1);
    }

    pub fn cursor_down(&mut self, rope: &Rope) {
        self.true_cursor.y = (self.true_cursor.y + 1).min(rope.len_lines().saturating_sub(1));
    }
    pub fn cursor_left(&mut self, _rope: &Rope) {
        self.true_cursor.x = self.true_cursor.y.saturating_sub(1);
        self.extend = self.true_cursor.x;
    }
    pub fn cursor_right(&mut self, rope: &Rope) {
        self.true_cursor.x =
            (self.true_cursor.x + 1).min(rope.line(self.true_cursor.y).len_chars() - 1);
        self.extend = self.true_cursor.x;
    }

    pub fn cursor_extend(&mut self, rope: &Rope) {
        self.extend = rope.get_line(self.true_cursor.y).unwrap().len_chars();
    }

    pub fn set_line<'a, 'b>(&'a mut self, line: &'b str, rope: &Rope) {
        if line != "0" {
            self.true_cursor.y = line
                .parse::<usize>()
                .unwrap()
                .saturating_sub(1)
                .min(rope.len_lines() - 1);
            self.true_cursor.x = 0;
        }
    }

    pub fn handle_scroll(&mut self) {
        if self.true_cursor.y < self.scroll.y {
            self.scroll.y = self.true_cursor.y;
        }
        if let Some(last_rect) = *self.last_pos.lock().unwrap() {
            if self.true_cursor.y > (self.scroll.y + last_rect.height as usize - 1) {
                self.scroll.y = self.true_cursor.y - last_rect.height as usize + 1;
            }
        }
    }
}
