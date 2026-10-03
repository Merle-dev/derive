use crate::App;

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
    Command(CommandEvent),
    Error(anyhow::Error),
    Lsp,
}

impl App {
    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::Lsp => self.process_lsp(),
            Event::Command(cmd) => self.process_command(cmd),
            Event::Error(err) => self.process_error(err),
        }
    }
    fn process_command(&mut self, commad_event: CommandEvent) {
        match commad_event.cmd.as_str() {
            "quit" => self.editor.quit = true,
            "normal" => self.editor.into_normal_mode(),
            "insert" => self.editor.into_insert_mode(),
            "visual" => self.editor.into_visual_mode(),
            "into_search" => {
                self.editor.bartask = crate::components::bottombar::BottomBarTask::Search
            }
            "into_command_line" => {
                self.editor.bartask = crate::components::bottombar::BottomBarTask::Command
            }
            _ => self.compositor.process_command_event(
                commad_event,
                &mut crate::Context {
                    editor: &mut self.editor,
                },
            ),
        }
    }
    fn process_error(&mut self, err: anyhow::Error) {}
    fn process_lsp(&mut self) {}
}

fn merge_nums(nums: Vec<u8>) -> usize {
    nums.into_iter()
        .enumerate()
        .rev()
        .fold(0, |a, (i, n)| a + (n as usize) * 10usize.pow(i as u32))
}
