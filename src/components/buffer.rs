use std::sync::Mutex;

use anyhow::Context as AnyhowContext;
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::{Paragraph, Widget},
};
use ropey::Rope;

use crate::{Context, components::UIComponentTrait, editor::DocumentId, keys::Mode};

pub struct TextBuffer {
    pub cursor: Position,
    pub clamp_x: u16,
    pub scroll: usize,
    pub id: DocumentId,
    pub last_pos: Mutex<Option<Rect>>,
}

impl TextBuffer {
    pub fn new(document_id: DocumentId) -> Self {
        Self {
            cursor: Position::default(),
            clamp_x: 0,
            scroll: 0,
            id: document_id,
            last_pos: Mutex::new(None),
        }
    }
    pub fn get_rope<'a, 'b>(&'a self, ctx: &'b mut Context) -> Option<&'b mut Rope> {
        let id = self.id.clone();
        ctx.editor.buffers.get_mut(&id).map(|doc| &mut doc.rope)
    }
    pub fn update_last_pos(&self, rect: Rect) {
        let mut last_pos = self.last_pos.lock().unwrap();
        *last_pos = Some(rect);
    }

    pub fn update_cursor<'a, 'b>(&'a self, ctx: &'b mut Context) {
        if let Some(last_rect) = *self.last_pos.lock().unwrap() {
            ctx.editor.cursor = Some(Position {
                x: (self.clamp_x + last_rect.x)
                    .min(last_rect.x + last_rect.width.saturating_sub(1)),
                y: (self.cursor.y + last_rect.y)
                    .min(last_rect.y + last_rect.height.saturating_sub(1)),
            });
        }
    }

    pub fn update_clamp_x(&mut self, rope: &Rope) -> anyhow::Result<()> {
        self.clamp_x = self.cursor.x.min(
            rope.get_line(self.cursor.y as usize)
                .context("No such line in rope")?
                .len_chars()
                .saturating_sub(if rope.len_lines() == self.cursor.y as usize + 1 {
                    0
                } else {
                    1
                }) as u16,
        );
        Ok(())
    }

    pub fn cursor_up(&mut self, rope: &Rope) -> anyhow::Result<()> {
        self.cursor.y = self.cursor.y.saturating_sub(1);
        self.update_clamp_x(rope)
    }
    pub fn cursor_down(&mut self, rope: &Rope) -> anyhow::Result<()> {
        self.cursor.y = self
            .cursor
            .y
            .saturating_add(1)
            .min(rope.len_lines() as u16 - 1);

        self.update_clamp_x(rope)
    }
    pub fn cursor_left(&mut self, rope: &Rope) -> anyhow::Result<()> {
        if self.cursor.x > self.clamp_x {
            self.cursor.x = self.clamp_x.saturating_sub(1);
        } else {
            self.cursor.x = self.cursor.x.saturating_sub(1);
        }
        self.update_clamp_x(rope)
    }
    pub fn cursor_right(&mut self, rope: &Rope) -> anyhow::Result<()> {
        self.cursor.x = self.cursor.x.saturating_add(1);
        self.update_clamp_x(rope)?;
        self.cursor.x = self.clamp_x;
        Ok(())
    }

    pub fn get_index(&self, rope: &Rope) -> usize {
        let lines = rope
            .lines()
            .map(|line| line.len_chars())
            .collect::<Vec<usize>>();
        lines[..self.cursor.y as usize].iter().sum::<usize>() + self.clamp_x as usize
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
        match key_event.code {
            KeyCode::Char(c) if mode == Mode::Insert => {
                let index = self.get_index(&rope);
                rope.insert_char(index, c);
                self.cursor_right(&rope).unwrap();
                None
            }
            KeyCode::Backspace if self.cursor != Position::default() && mode == Mode::Insert => {
                let lines = rope
                    .lines()
                    .map(|line| line.len_chars())
                    .collect::<Vec<usize>>();
                let index =
                    lines[..self.cursor.y as usize].iter().sum::<usize>() + self.clamp_x as usize;

                let prev_linelen_snapshot = rope
                    .get_line(self.cursor.y.saturating_sub(1) as usize)
                    .unwrap()
                    .len_chars() as u16
                    - 1;

                rope.remove(index.saturating_sub(1)..index);
                if self.cursor.x == 0 {
                    self.cursor.y -= 1;
                    self.cursor.x = prev_linelen_snapshot;
                    self.update_clamp_x(&rope).unwrap();
                } else {
                    self.cursor_left(&rope).unwrap();
                }
                None
            }
            KeyCode::Backspace if mode == Mode::Insert => None,
            KeyCode::Delete if mode == Mode::Insert => {
                let index = self.get_index(&rope);
                rope.remove(index..(index + 1).min(rope.len_chars()));
                None
            }
            KeyCode::Enter if mode == Mode::Insert => {
                let index = self.get_index(&rope);
                rope.insert_char(index, '\n');
                self.cursor.x = 0;
                self.cursor.y += 1;
                self.update_clamp_x(&rope).unwrap();
                None
            }
            KeyCode::End => {
                let lines = rope
                    .lines()
                    .map(|line| line.len_chars())
                    .collect::<Vec<usize>>();
                self.cursor.x = *lines.get(self.cursor.y as usize).unwrap() as u16;
                self.update_clamp_x(&rope).unwrap();
                None
            }
            KeyCode::Home => {
                self.cursor.x = 0;
                self.update_clamp_x(&rope).unwrap();
                None
            }
            _ => Some(key_event),
        }
    }
    fn command_input(&mut self, command: crate::event::CommandEvent, ctx: &mut Context) {
        let rope = self.get_rope(ctx).unwrap();
        match command.cmd.as_str() {
            "up" => self.cursor_up(&rope),
            "down" => self.cursor_down(&rope),
            "left" => self.cursor_left(&rope),
            "right" => self.cursor_right(&rope),
            _ => Ok(()),
        }
        .unwrap();
        self.update_cursor(ctx);
    }

    fn render(&self, area: Rect, buffer: &mut Buffer, ctx: &mut Context) {
        let max_line_num = (area.height as usize + self.scroll + 1).ilog10() as usize;
        let rope = self.get_rope(ctx).unwrap();

        for (index, line) in (0..area.height as usize)
            .map(|i| rope.get_line(i + self.scroll))
            .enumerate()
        {
            let area = Rect {
                y: area.y + index as u16,
                height: 1,
                ..area
            };
            let line_num = index + self.scroll + 1;
            let line_num_spacing = " ".repeat(max_line_num - line_num.ilog10() as usize);
            if let Some(line) = line {
                Paragraph::new(format!(
                    " {line_num_spacing}{line_num}   {}",
                    line.to_string()
                ))
                .render(area, buffer);
                // } else {
                // Paragraph::new(format!(" {line_num_spacing}{line_num}"))
            }
        }
        self.update_last_pos(Rect {
            x: area.x + 6,
            ..area
        });
        self.update_cursor(ctx);
    }
}
