use ratatui::{buffer::Buffer, layout::Size};

use crate::{
    Context,
    components::UIComponentTrait,
    compositor::{ComponentPosition, Compositor, CompositorItem},
};

impl Compositor {
    pub fn new(size: Size) -> Self {
        Self {
            components: vec![],
            input_component: None,
            size,
        }
    }
    pub fn add(&mut self, position: ComponentPosition, component: impl UIComponentTrait + 'static) {
        self.components.push(CompositorItem {
            component: Box::new(component),
            position,
        });
    }

    pub fn render(&self, buffer: &mut Buffer, ctx: &mut Context) {
        let area_map = self.calculate_visibility(ctx);
        for (index, comoponent) in self.components.iter().enumerate() {
            if let Some(area) = area_map.get(&index)
                && comoponent.component.redraw(ctx)
            {
                comoponent.component.render(*area, buffer, ctx);
            }
        }
    }
}
