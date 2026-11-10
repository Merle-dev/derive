use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{Context, components::buffer::TextBuffer};

impl TextBuffer {
    pub fn render_internal(&self, area: Rect, buffer: &mut Buffer, ctx: &mut Context) {
        let max_line_num = (area.height as usize + self.scroll.y + 1).ilog10() as usize;
        let rope = self.get_rope(ctx).unwrap();

        for (index, line) in (0..area.height as usize)
            .map(|i| rope.get_line(i + self.scroll.y))
            .enumerate()
        {
            let area = Rect {
                y: area.y + index as u16,
                height: 1,
                ..area
            };
            let line_num = index + self.scroll.y + 1;
            let line_num_spacing = " ".repeat(max_line_num - line_num.ilog10() as usize);
            if let Some(line) = line {
                Paragraph::new(format!(
                    " {line_num_spacing}{line_num}   {}",
                    line.to_string()
                ))
                .render(area, buffer);
            }
        }
        self.update_last_pos(Rect {
            x: area.x + max_line_num as u16 + 5,
            ..area
        });
        self.update_app_cursor(ctx);
    }
}
