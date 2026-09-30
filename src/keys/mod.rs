use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::event::CommandEvent;

mod loader;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Key {
    code: KeyCode,
    modification: KeyModifiers,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum KeyMapIdent {
    AnyNum,
    Key(Key),
}

#[derive(Clone, Debug)]
enum KeyMapMember {
    Map(Arc<HashMap<KeyMapIdent, KeyMapMember>>),
    Value(String),
}

#[derive(Clone, Debug)]
pub struct KeyMap {
    modes: HashMap<Mode, Arc<HashMap<KeyMapIdent, KeyMapMember>>>,
    current_map: Arc<HashMap<KeyMapIdent, KeyMapMember>>,
    current_default: Mode,
    nums: Vec<u8>,
}

impl KeyMap {
    pub fn new() -> Self {
        let modes = HashMap::from([(Mode::Normal, Arc::new(HashMap::new()))]);
        let current_map = modes.get(&Mode::Normal).unwrap().clone();
        Self {
            modes,
            current_map,
            current_default: Mode::Normal,
            nums: vec![],
        }
    }
    pub fn process(&mut self, key_event: KeyEvent) -> Option<CommandEvent> {
        let key = match key_event.code {
            KeyCode::Char(n)
                if (48..57).contains(&(n as u8))
                    && self.current_map.get(&KeyMapIdent::AnyNum).is_some() =>
            {
                self.nums.push(n as u8 - 48);
                return None;
            }
            _ => KeyMapIdent::Key(Key {
                code: key_event.code,
                modification: key_event.modifiers,
            }),
        };

        if let Some(member) = self.current_map.get(&key).cloned() {
            match member {
                KeyMapMember::Value(cmd) => {
                    let nums = self.nums.clone();
                    self.reset();
                    return Some(CommandEvent {
                        cmd,
                        args: crate::event::StringOrNums::Nums(nums),
                    });
                }

                KeyMapMember::Map(map) => self.current_map = map.clone(),
            }
        } else {
            self.reset();
        }
        None
    }

    fn pm(map: Arc<HashMap<KeyMapIdent, KeyMapMember>>) -> String {
        [
            "{".to_string(),
            map.iter()
                .map(|(i, m)| {
                    format!(
                        "{}: {}",
                        match i {
                            KeyMapIdent::AnyNum => "*".to_string(),
                            KeyMapIdent::Key(k) => k.code.to_string(),
                        },
                        match m {
                            KeyMapMember::Map(map) => Self::pm(map.clone()),
                            KeyMapMember::Value(v) => format!("{v:?}"),
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join(" | "),
            "}".to_string(),
        ]
        .concat()
    }

    pub fn simple_current_map(&self) -> String {
        Self::pm(self.current_map.clone())
    }

    pub fn set_mode(&mut self, mode: Mode) {
        if let Some(map) = self.modes.get(&mode) {
            self.current_default = mode;
            self.current_map = map.clone();
        }
    }

    pub fn reset(&mut self) {
        self.current_map = self.modes.get(&self.current_default).unwrap().clone();
        self.nums.clear();
    }
}
