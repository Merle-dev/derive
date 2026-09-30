use crossterm::event::KeyEvent;

use crate::compositor::Compositor;

impl Compositor {
    pub fn set_input(&mut self, index: Option<usize>) {
        self.input_component = index;
    }
    pub fn process_key_event(
        &mut self,
        key_event: KeyEvent,
        ctx: &mut crate::Context,
    ) -> Option<KeyEvent> {
        self.input_component
            .and_then(|i| {
                self.components
                    .get_mut(i)
                    .and_then(|comp| comp.component.kb_input(key_event, ctx))
            })
            .or(Some(key_event))
    }
}
