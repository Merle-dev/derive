use std::any::Any;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::Context;

pub mod buffer;
// pub mod buffer_menu;
pub mod bottombar;
pub mod topbar;

pub trait UIComponentTrait: Any {
    fn redraw(&self, ctx: &mut Context) -> bool {
        true
    }
    fn kb_input(&mut self, key_event: KeyEvent, ctx: &mut Context) -> Option<KeyEvent> {
        Some(key_event)
    }
    fn captures_input(&self, ctx: &mut Context) -> bool {
        false
    }

    fn is_visible(&self, ctx: &mut Context) -> bool;
    fn render(&self, area: Rect, buffer: &mut Buffer, ctx: &mut Context);
}
