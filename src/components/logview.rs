use ratatui::widgets::{List, ListItem, Widget};

use crate::components::UIComponentTrait;

pub struct LogView;

impl UIComponentTrait for LogView {
    fn is_visible(&self, ctx: &mut crate::Context) -> bool {
        ctx.editor.logview
    }

    fn render(
        &self,
        area: ratatui::prelude::Rect,
        buffer: &mut ratatui::prelude::Buffer,
        ctx: &mut crate::Context,
    ) {
        ctx.editor.cursor = None;
        List::new(
            ctx.log[ctx.log.len().saturating_sub(area.height as usize)..]
                .iter()
                .cloned()
                .map(ListItem::from)
                .collect::<Vec<ListItem>>(),
        )
        .render(area, buffer);
    }
    fn captures_input(&self, ctx: &mut crate::Context) -> bool {
        ctx.editor.logview
    }
    fn redraw(&self, ctx: &mut crate::Context) -> bool {
        ctx.editor.logview
    }
}
