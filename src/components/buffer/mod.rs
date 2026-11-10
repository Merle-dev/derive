use std::sync::Mutex;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ropey::Rope;

mod cursor;
mod movement;
mod render;

use crate::{Context, components::UIComponentTrait, editor::DocumentId, keys::Mode};

#[derive(Default, PartialEq)]
pub struct TruePosition {
    x: usize,
    y: usize,
}

pub struct TextBuffer {
    show_cursor: Position,
    true_cursor: TruePosition,
    scroll: TruePosition,
    extend: usize,

    id: DocumentId,
    last_pos: Mutex<Option<Rect>>,
}

impl TextBuffer {
    pub fn new(document_id: DocumentId) -> Self {
        Self {
            show_cursor: Position::default(),
            true_cursor: TruePosition::default(),
            scroll: TruePosition::default(),
            extend: 0,
            id: document_id,
            last_pos: Mutex::new(None),
        }
    }

    pub fn get_rope<'a, 'b>(&'a self, ctx: &'b mut Context) -> Option<&'b mut Rope> {
        let id = self.id.clone();
        ctx.editor.buffers.get_mut(&id).map(|doc| &mut doc.rope)
    }

    pub(super) fn update_last_pos(&self, rect: Rect) {
        let mut last_pos = self.last_pos.lock().unwrap();
        *last_pos = Some(rect);
    }

    pub(super) fn update_show_cursor(&mut self) {
        self.show_cursor = Position {
            x: (self.true_cursor.x.min(self.extend) - self.scroll.x) as u16,
            y: (self.true_cursor.y - self.scroll.y) as u16,
        }
    }

    pub(super) fn update_app_cursor<'a, 'b>(&'a self, ctx: &'b mut Context) {
        if let Some(last_rect) = *self.last_pos.lock().unwrap() {
            ctx.editor.cursor = Some((self.show_cursor, last_rect));
        }
    }

    pub fn update_cursor(&mut self, ctx: &mut Context) {
        let rope = self.get_rope(ctx).unwrap();
        self.handle_scroll();
        self.cursor_extend(rope);
        self.update_show_cursor();
        self.update_app_cursor(ctx);
    }
}

impl UIComponentTrait for TextBuffer {
    fn is_visible(&self, _ctx: &mut Context) -> bool {
        true
    }

    fn captures_input(&self, _ctx: &mut Context) -> bool {
        true
    }
    fn kb_input(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        ctx: &mut Context,
    ) -> Option<crossterm::event::KeyEvent> {
        let mode = ctx.editor.mode.clone();
        let rope = self.get_rope(ctx).unwrap();
        let ret = match (key_event.code, mode) {
            (KeyCode::Char(c), Mode::Insert) => {
                let char_idx =
                    rope.line_to_char(self.true_cursor.y) + self.true_cursor.x.min(self.extend);

                rope.insert(char_idx, &c.to_string());
                self.true_cursor.x += 1;
                None
            }
            (KeyCode::Backspace, Mode::Insert) => {
                let char_idx = rope.line_to_char(self.true_cursor.y) + self.true_cursor.x - 1;
                if self.true_cursor != TruePosition::default() {
                    if self.true_cursor.x == 0 {
                        self.true_cursor.y = self.true_cursor.y.saturating_sub(1);
                        self.true_cursor.x = rope.line(self.true_cursor.y).len_chars() - 1;
                    } else {
                        self.true_cursor.x -= 1;
                    }
                }

                let _ = rope.try_remove(char_idx..=char_idx);
                None
            }
            (KeyCode::Delete, Mode::Insert) => {
                let char_idx = rope.line_to_char(self.true_cursor.y) + self.true_cursor.x;

                let _ = rope.try_remove(char_idx..=char_idx);
                None
            }
            (KeyCode::Enter, Mode::Insert) => {
                let char_idx = rope.try_line_to_char(self.true_cursor.y).ok()?
                    + self.true_cursor.x.min(self.extend);
                rope.insert(char_idx, "\n");

                self.true_cursor.x = 0;
                self.true_cursor.y += 1;

                None
            }
            (KeyCode::End, _) => {
                self.true_cursor.x = rope.line(self.true_cursor.y).len_chars() - 1;
                None
            }
            (KeyCode::Home, _) => {
                self.true_cursor.x = 0;
                None
            }
            _ => Some(key_event),
        };
        self.update_cursor(ctx);
        ret
    }
    fn command_input(&mut self, command: crate::event::CommandEvent, ctx: &mut Context) {
        let rope = self.get_rope(ctx).unwrap();
        match command.cmd.as_str() {
            "up" => self.cursor_up(&rope),
            "down" => self.cursor_down(&rope),
            "left" => self.cursor_left(&rope),
            "right" => self.cursor_right(&rope),
            line if line.contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) => {
                self.set_line(line, rope)
            }
            _ => (),
        };

        self.update_cursor(ctx);
    }

    fn render(&self, area: Rect, buffer: &mut Buffer, ctx: &mut Context) {
        self.render_internal(area, buffer, ctx);
    }
}
