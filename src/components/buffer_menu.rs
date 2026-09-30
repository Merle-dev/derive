use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
};

const PADDING: usize = 8;

impl Into<MenuLine> for String {
    fn into(self) -> MenuLine {
        MenuLine::Single(self)
    }
}

impl Into<MenuLine> for (String, String) {
    fn into(self) -> MenuLine {
        MenuLine::Double(self.0, self.1)
    }
}

enum MenuLine {
    Single(String),
    Double(String, String),
}

pub struct TextBufferMenu {
    pub height: u16,
    pub scroll: usize,
    pub selected: Option<usize>,
    pub longest_element_len: usize,
    pub lines: Vec<MenuLine>,
}

impl TextBufferMenu {
    fn new<T: Clone + Into<MenuLine>>(height: u16, lines: Vec<T>) -> Self {
        Self {
            height,
            scroll: 0,
            selected: None,
            longest_element_len: lines.iter().fold(0, |acc, item| {
                acc.max(match item.clone().into() {
                    MenuLine::Single(s) => s.len(),
                    MenuLine::Double(a, b) => a.len() + b.len(),
                })
            }),
            lines: lines
                .into_iter()
                .map(|l| l.into())
                .collect::<Vec<MenuLine>>(),
        }
    }

    pub fn string_lines(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|line| match line {
                MenuLine::Single(str) => {
                    let filler = " ".repeat(self.longest_element_len - str.len());
                    format!("{str}{filler}")
                }
                MenuLine::Double(name, description) => {
                    let filler = " ".repeat(
                        PADDING + self.longest_element_len - name.len() - description.len(),
                    );
                    format!("{name}{filler}{description}")
                }
            })
            .collect()
    }
    pub fn menu_position(&self, buffer_area: Rect, prefered_x: u16, prefered_y: u16) -> Rect {}
}
