use ratatui::widgets::{Paragraph, Widget};

use crate::components::UIComponentTrait;

pub struct TopBar;

impl UIComponentTrait for TopBar {
    fn redraw(&self, ctx: &mut crate::Context) -> bool {
        true
    }

    fn is_visible(&self, ctx: &mut crate::Context) -> bool {
        true
    }

    fn render(
        &self,
        area: ratatui::prelude::Rect,
        buffer: &mut ratatui::prelude::Buffer,
        ctx: &mut crate::Context,
    ) {
        let str = ctx
            .editor
            .buffers
            .iter()
            .map(|(_, doc)| doc.path.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join(" | ");

        Paragraph::new(str).render(area, buffer);
    }
}
