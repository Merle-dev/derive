use std::collections::HashMap;

use ratatui::layout::{Constraint, Rect};

use crate::{
    Context,
    compositor::{ComponentPosition, Compositor},
};

impl Compositor {
    pub fn calculate_visibility(&self, ctx: &mut Context) -> HashMap<usize, Rect> {
        let mut area_map = HashMap::new();
        let mut positions =
            self.components
                .iter()
                .enumerate()
                .fold(vec![], |mut acc, (idx, item)| {
                    if item.component.is_visible(ctx) {
                        acc.push((idx, item.position.clone()));
                    }
                    acc
                });

        positions.sort_by(|a, b| a.1.cmp(&b.1));
        let area = Rect {
            x: 0,
            y: 0,
            width: self.size.width,
            height: self.size.height,
        };

        pub fn split(
            rect: Rect,
            position: ComponentPosition,
            positions: &Vec<(usize, ComponentPosition)>,
            area_map: &mut HashMap<usize, Rect>,
            format_fn: impl FnOnce(Rect) -> (Rect, Rect),
        ) -> Rect {
            let Some((index, _)) = positions.iter().find(|(_, p)| p == &position) else {
                return rect;
            };
            let (main, split_off) = format_fn(rect);
            area_map.insert(*index, split_off);
            main
        }
        let _ = split(
            area,
            ComponentPosition::Floating,
            &positions,
            &mut area_map,
            |rect| {
                let floating =
                    rect.centered(Constraint::Percentage(75), Constraint::Percentage(80));
                (Rect::default(), floating)
            },
        );
        let area = split(
            area,
            ComponentPosition::TopBar,
            &positions,
            &mut area_map,
            |Rect {
                 x, width, height, ..
             }| {
                (
                    Rect::new(x, 1, width, height - 1),
                    Rect::new(x, 0, width, 1),
                )
            },
        );
        let area = split(
            area,
            ComponentPosition::BottomBar,
            &positions,
            &mut area_map,
            |Rect {
                 x,
                 y,
                 width,
                 height,
             }| {
                (
                    Rect::new(x, y, width, height - 1),
                    Rect::new(x, height, width, 1),
                )
            },
        );
        let area = split(
            area,
            ComponentPosition::Leftpanel,
            &positions,
            &mut area_map,
            |Rect {
                 x,
                 y,
                 width,
                 height,
             }| {
                let r = (self.size.width as f32 * 0.2).round() as u16;
                (
                    Rect::new(x + r, y, width - r, height),
                    Rect::new(x, y, r, height),
                )
            },
        );
        let area = split(
            area,
            ComponentPosition::Rightpanel,
            &positions,
            &mut area_map,
            |Rect {
                 x,
                 y,
                 width,
                 height,
             }| {
                let r = (self.size.width as f32 * 0.25).round() as u16;
                (
                    Rect::new(x, y, width - r, height),
                    Rect::new(x + width - r, y, r, height),
                )
            },
        );

        let _ = split(
            area,
            ComponentPosition::Main,
            &positions,
            &mut area_map,
            |rect| (Rect::default(), rect),
        );

        area_map
    }
}
