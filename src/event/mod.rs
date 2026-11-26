use crate::{App, editor::LspConnection, lsp_server::spawn_server};

pub enum StringOrNums {
    String(String),
    Nums(Vec<u8>),
}

impl TryInto<String> for StringOrNums {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(match self {
            StringOrNums::String(s) => s,
            StringOrNums::Nums(ns) => merge_nums(ns).to_string(),
        })
    }
}

impl TryInto<usize> for StringOrNums {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<usize, Self::Error> {
        Ok(match self {
            StringOrNums::Nums(ns) => merge_nums(ns),
            StringOrNums::String(s) => s.parse()?,
        })
    }
}

pub struct CommandEvent {
    pub cmd: String,
    pub args: StringOrNums,
}

pub enum Event {
    Terminal(crossterm::event::Event),
    Command(CommandEvent),
    Error(anyhow::Error),
    Info(String),
    Lsp(String),
}

impl App {
    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::Terminal(ev) => self.process_terminal_event(ev),
            Event::Command(cmd) => self.process_command(cmd),
            Event::Error(err) => self.process_error(err),
            Event::Lsp(str) => self.process_lsp(str),
            Event::Info(info) => {
                self.log.push(info);
            }
        }
    }
    fn process_terminal_event(&mut self, terminal_event: crossterm::event::Event) {
        match terminal_event {
            crossterm::event::Event::Key(key_event) => {
                if let Some(cmd_event) = self
                    .process_key_event(key_event)
                    .and_then(|key_event| self.editor.key_map.process(key_event))
                {
                    self.process_event(Event::Command(cmd_event));
                }
            }
            _ => (),
        }
    }
    fn process_command(&mut self, commad_event: CommandEvent) {
        match commad_event.cmd.as_str() {
            "quit" | "q" => self.editor.quit = true,
            "write" | "w" => self.editor.write().unwrap(),
            "wq" => {
                if self.editor.write().is_ok() {
                    self.editor.quit = true;
                }
            }
            "normal" => self.editor.into_normal_mode(),
            "insert" => self.editor.into_insert_mode(),
            "visual" => self.editor.into_visual_mode(),
            "into_search" => {
                self.editor.bartask = crate::components::bottombar::BottomBarTask::Search
            }
            "into_command_line" => {
                self.editor.bartask = crate::components::bottombar::BottomBarTask::Command
            }
            "toggle_log" => self.editor.logview = !self.editor.logview,
            "spawn_lsp" => {
                let name = "rust-analyzer".to_string();
                let sender = spawn_server(name.clone(), self.sender.clone()).unwrap();
                sender.send(crate::lsp_server::EditorRequest::Init).unwrap();
                self.editor
                    .lsp_connections
                    .push(LspConnection { name, sender });
            }
            _ => self.compositor.process_command_event(
                commad_event,
                &mut crate::Context {
                    editor: &mut self.editor,
                    sender: &mut self.sender,
                    log: &self.log,
                },
            ),
        }
    }
    fn process_error(&mut self, err: anyhow::Error) {
        format!("ERROR: {:#?}", err)
            .lines()
            .for_each(|l| self.log.push(l.to_string()));
    }
    fn process_lsp(&mut self, str: String) {
        format!("LSP: {:#?}", str)
            .lines()
            .for_each(|l| self.log.push(l.to_string()));
    }
}

fn merge_nums(nums: Vec<u8>) -> usize {
    nums.into_iter()
        .enumerate()
        .rev()
        .fold(0, |a, (i, n)| a + (n as usize) * 10usize.pow(i as u32))
}
