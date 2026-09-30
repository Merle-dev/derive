use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{Context, components::UIComponentTrait, keys::Mode};

pub struct TextBuffer {
    pub t: String,
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
        match key_event.code {
            crossterm::event::KeyCode::Char(c) if ctx.editor.mode == Mode::Insert => {
                self.t.push(c);
                None
            }
            _ => Some(key_event),
        }
    }

    fn render(&self, area: Rect, buffer: &mut Buffer, ctx: &mut Context) {
        fn to_ciff(c: usize) -> Vec<u8> {
            if c < 10 {
                vec![c as u8]
            } else {
                let div10 = (c as f32 * 0.1).floor();
                let last_ciff = c as f32 - div10 * 10.0;
                [to_ciff(div10 as usize), vec![last_ciff as u8]].concat()
            }
        }

        let s: usize = 0;
        let ml = to_ciff(s + area.height as usize).len();

        for y in s..s + area.height as usize {
            to_ciff(y).iter().rev().enumerate().for_each(|(x, c)| {
                buffer
                    .cell_mut((area.x + (ml - x) as u16, (area.y as usize + y - s) as u16))
                    .map(|cell| cell.set_char((*c + 48) as char));
            });
        }
        Paragraph::new(self.t.clone()).render(area, buffer);
    }
}
