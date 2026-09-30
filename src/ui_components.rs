use ratatui::layout::Rect;

use crate::{
    Context,
    rendering::{Buffer, Cell, Style},
};

pub trait UiComponent: Send + Sync + 'static {
    fn needs_redraw(&self, ctx: Context) -> bool {
        true
    }
    fn render(&self, area: Rect, buffer: &mut Buffer, ctx: Context);
}

pub struct Line {
    style: Style,
    text: String,
}

impl Line {
    pub fn new(text: String) -> Self {
        Self {
            style: Style::default(),
            text,
        }
    }
}

impl UiComponent for Line {
    fn render(&self, area: Rect, buffer: &mut Buffer, ctx: Context) {
        let index = (area.y * buffer.width + area.x) as usize;
        self.text[..(area.width as usize).min(self.text.len())]
            .chars()
            .enumerate()
            .for_each(|(i, ch)| {
                let cell = Cell {
                    style: self.style.clone(),
                    symbol: ch,
                };
                buffer.cells[index + i] = cell;
            });
    }
}
