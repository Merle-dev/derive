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
    input_component: Option<usize>,
    size: Size,
}
