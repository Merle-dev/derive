use crossterm::event::KeyCode;
use ratatui::widgets::{Paragraph, Widget};

use crate::{components::UIComponentTrait, keys::Mode};

#[derive(PartialEq)]
pub enum BottomBarTask {
    Display,
    Command,
    Search,
    Error,
}

pub struct BottomBar {
    input: String,
}

impl BottomBar {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}

impl UIComponentTrait for BottomBar {
    fn redraw(&self, _ctx: &mut crate::Context) -> bool {
        true
    }

    fn captures_input(&self, ctx: &mut crate::Context) -> bool {
        ctx.editor.bartask == BottomBarTask::Command || ctx.editor.bartask == BottomBarTask::Search
    }
    fn kb_input(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        ctx: &mut crate::Context,
    ) -> Option<crossterm::event::KeyEvent> {
        match ctx.editor.bartask {
            BottomBarTask::Command | BottomBarTask::Search => match key_event.code {
                KeyCode::Char(ch) => {
                    self.input.push(ch);
                    None
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    None
                }
                KeyCode::Enter => {
                    ctx.editor.bartask = BottomBarTask::Display;
                    None
                }
                _ => Some(key_event),
            },
            _ => Some(key_event),
        }
    }

    fn is_visible(&self, _ctx: &mut crate::Context) -> bool {
        true
    }

    fn render(
        &self,
        area: ratatui::prelude::Rect,
        buffer: &mut ratatui::prelude::Buffer,
        ctx: &mut crate::Context,
    ) {
        match ctx.editor.bartask {
            BottomBarTask::Display => {
                Paragraph::new(match ctx.editor.mode {
                    Mode::Normal => "normal",
                    Mode::Insert => "insert",
                    Mode::Visual => "visual",
                })
                .render(area, buffer);
                if let Some(cursor) = ctx.editor.cursor {
                    Paragraph::new(format!("{}:{}", cursor.x, cursor.y))
                        .right_aligned()
                        .render(area, buffer);
                }
            }
            BottomBarTask::Search => {
                Paragraph::new(format!("search: {}", self.input)).render(area, buffer)
            }
            BottomBarTask::Command => {
                Paragraph::new(format!(": {}", self.input)).render(area, buffer)
            }
            _ => (),
        }
    }
}
