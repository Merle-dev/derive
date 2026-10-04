use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Size};

use crate::{
    components::{bottombar, buffer, topbar},
    compositor::{ComponentPosition, Compositor},
    editor::Editor,
    event::Event,
};

mod components;
mod compositor;
mod editor;
mod event;
mod keys;

struct Context<'e> {
    editor: &'e mut Editor,
    sender: &'e mut kanal::Sender<Event>,
}

struct App {
    compositor: Compositor,
    sender: kanal::Sender<Event>,
    receiver: kanal::Receiver<Event>,
    editor: Editor,
}

impl App {
    pub fn new(size: Size) -> anyhow::Result<Self> {
        let (sender, receiver) = kanal::bounded(16);
        let key_sender = sender.clone();
        std::thread::spawn(move || {
            loop {
                key_sender
                    .send(Event::Terminal(crossterm::event::read().unwrap()))
                    .unwrap();
            }
        });
        Ok(Self {
            compositor: Compositor::new(size),
            editor: Editor::new()?,
            receiver,
            sender,
        })
    }

    pub fn render(&mut self, buffer: &mut Buffer) {
        let App {
            compositor,
            editor,
            sender,
            ..
        } = self;
        compositor.calculate_visibility(&mut Context { editor, sender });
        compositor.render(buffer, &mut Context { editor, sender });
        compositor.update_capture_components(&mut Context { editor, sender });
    }

    pub fn process_key_event(&mut self, key_event: KeyEvent) -> Option<KeyEvent> {
        let App {
            compositor,
            editor,
            sender,
            ..
        } = self;

        compositor.process_key_event(key_event, &mut Context { editor, sender })
    }
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let path = PathBuf::from(args.get(1).unwrap());
    ratatui::run(|terminal| {
        let mut app = App::new(terminal.size()?)?;
        let doc_id = app.editor.add_doc(path)?;

        app.compositor
            .add(ComponentPosition::Main, buffer::TextBuffer::new(doc_id));
        app.compositor
            .add(ComponentPosition::TopBar, topbar::TopBar);
        app.compositor
            .add(ComponentPosition::BottomBar, bottombar::BottomBar::new());

        while !app.editor.quit {
            if app.editor.cursor.is_some() {
                terminal.show_cursor()?;
            } else {
                terminal.hide_cursor()?;
            }
            terminal.draw(|frame| {
                app.render(frame.buffer_mut());
                if let Some(cursor) = app.editor.cursor {
                    frame.set_cursor_position(cursor);
                }
            })?;

            app.process_event(app.receiver.recv()?);
        }
        anyhow::Ok(())
    })?;
    Ok(())
}
