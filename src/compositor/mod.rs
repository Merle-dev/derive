use std::collections::HashMap;

use ratatui::layout::Size;

use crate::components::UIComponentTrait;

mod input_processing;
mod layout;
mod rendering;

pub struct CompositorItem {
    component: Box<dyn UIComponentTrait>,
    position: ComponentPosition,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentPosition {
    TopBar,
    BottomBar,
    Leftpanel,
    Rightpanel,
    Floating,
    Main,
}

pub struct Compositor {
    components: Vec<CompositorItem>,
    pub input_component_stack: Vec<usize>,
    size: Size,
}
