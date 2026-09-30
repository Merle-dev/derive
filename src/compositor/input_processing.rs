use std::collections::HashSet;

use crossterm::event::KeyEvent;

use crate::{Context, compositor::Compositor};

impl Compositor {
    pub fn update_capture_components(&mut self, ctx: &mut Context) {
        let mut capture_components = self
            .components
            .iter()
            .enumerate()
            .filter(|(_, item)| item.component.captures_input(ctx))
            .map(|(i, _)| i)
            .collect::<HashSet<usize>>();

        let mut input_component_stack = vec![];
        for index in &self.input_component_stack {
            if capture_components.remove(index) {
                input_component_stack.push(*index);
            }
        }
        input_component_stack.append(&mut capture_components.into_iter().collect());
        self.input_component_stack = input_component_stack;
    }

    pub fn add_input(&mut self, index: usize) {
        self.input_component_stack.push(index);
    }
    pub fn remove(&mut self, index: usize) {
        if index < self.components.len() {
            self.components.remove(index);
        }
    }
    pub fn process_key_event(
        &mut self,
        key_event: KeyEvent,
        ctx: &mut crate::Context,
    ) -> Option<KeyEvent> {
        if let Some(consumer) = self
            .input_component_stack
            .last()
            .and_then(|i| self.components.get_mut(*i))
        {
            consumer.component.kb_input(key_event, ctx)
        } else {
            Some(key_event)
        }
    }
}
