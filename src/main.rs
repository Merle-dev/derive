use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Size};

use crate::{
    components::{bottombar, buffer, topbar},
    compositor::{ComponentPosition, Compositor},
    editor::Editor,
};

mod components;
mod compositor;
mod editor;
mod event;
mod keys;

struct Context<'e> {
    editor: &'e mut Editor,
}

struct App {
    compositor: Compositor,
    editor: Editor,
}

impl App {
    pub fn new(size: Size) -> anyhow::Result<Self> {
        Ok(Self {
            compositor: Compositor::new(size),
            editor: Editor::new()?,
        })
    }

    pub fn render(&mut self, buffer: &mut Buffer) {
        let App { compositor, editor } = self;
        compositor.calculate_visibility(&mut Context { editor });
        compositor.render(buffer, &mut Context { editor });
        compositor.update_capture_components(&mut Context { editor });
    }

    pub fn process_key_event(&mut self, key_event: KeyEvent) -> Option<KeyEvent> {
        let App { compositor, editor } = self;

        compositor.process_key_event(key_event, &mut Context { editor })
    }
}

fn main() -> anyhow::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::new(terminal.size()?)?;
        app.compositor.add(
            ComponentPosition::Main,
            buffer::TextBuffer { t: "Main".into() },
        );
        app.compositor
            .add(ComponentPosition::TopBar, topbar::TopBar);
        app.compositor
            .add(ComponentPosition::BottomBar, bottombar::BottomBar::new());

        while !app.editor.quit {
            terminal.draw(|frame| app.render(frame.buffer_mut()))?;

            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => {
                    if let Some(cmd_event) = app
                        .process_key_event(key_event)
                        .and_then(|key_event| app.editor.key_map.process(key_event))
                    {
                        app.process_event(event::Event::Command(cmd_event));
                    }
                }
                _ => (),
            }
        }
        anyhow::Ok(())
    })?;
    Ok(())
}
