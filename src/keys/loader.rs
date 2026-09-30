use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Result, anyhow};
use crossterm::event::{KeyCode, KeyModifiers};

use crate::keys::{Key, KeyMap, KeyMapIdent, KeyMapMember, Mode};

#[derive(Eq, PartialEq, Debug)]
struct ConfLine {
    command: String,
    mode: Mode,
    keys: Vec<KeyMapIdent>,
}

#[derive(Clone, Debug)]
enum PreKeyMapMember<T: Clone> {
    Map(Box<HashMap<KeyMapIdent, PreKeyMapMember<T>>>),
    Value(T),
}

impl KeyMap {
    pub fn load(text: String) -> Result<Self> {
        let values = text.parse::<toml::Table>()?;
        let conf_lines = values
            .into_iter()
            .map(|(command, val)| {
                let table = val.as_table().context("After command must come a Table")?;
                Ok(ConfLine {
                    command,
                    mode: table
                        .get("mode")
                        .context("No Mode")
                        .and_then(|m| m.as_str().context("Mode has incorrect format"))
                        .and_then(|ms| match ms {
                            "n" => Ok(Mode::Normal),
                            "i" => Ok(Mode::Insert),
                            "v" => Ok(Mode::Visual),
                            m => Err(anyhow!("No such Mode as {m}")),
                        })?,
                    keys: table
                        .get("keys")
                        .context("No Keys")
                        .and_then(|keys| keys.as_array().context("Keys has incorrect format"))
                        .and_then(|keys| {
                            keys.iter()
                                .filter_map(|val| val.as_str())
                                .map(Self::str_to_keys)
                                .collect::<Result<Vec<KeyMapIdent>>>()
                        })?,
                })
            })
            .collect::<Result<Vec<ConfLine>>>()?;
        Self::from_vecs(conf_lines)
    }

    fn str_to_keys(str: &str) -> Result<KeyMapIdent> {
        let keycode = match str {
            "backspace" => Ok(KeyCode::Backspace),
            "enter" | "ret" => Ok(KeyCode::Enter),
            "left" => Ok(KeyCode::Left),
            "right" => Ok(KeyCode::Right),
            "up" => Ok(KeyCode::Up),
            "down" => Ok(KeyCode::Down),
            "home" => Ok(KeyCode::Home),
            "end" => Ok(KeyCode::End),
            "pageup" | "pgup" => Ok(KeyCode::PageUp),
            "pagedown" | "pgdown" => Ok(KeyCode::PageDown),
            "tab" => Ok(KeyCode::Tab),
            "backtab" => Ok(KeyCode::BackTab),
            "del" | "delete" => Ok(KeyCode::Delete),
            "ins" | "insert" => Ok(KeyCode::Insert),
            "esc" | "escape" => Ok(KeyCode::Esc),
            "space" => Ok(KeyCode::Char(' ')),
            "any" => return Ok(KeyMapIdent::AnyNum),
            k if k.starts_with('f') && k[1..].parse::<u8>().is_ok() => {
                Ok(KeyCode::F(k[1..].parse().unwrap()))
            }
            k if k.chars().count() == 1 => Ok(KeyCode::Char(k.chars().next().unwrap())),
            k => Err(anyhow!("{{{k}}} No such Key")),
        }?;

        Ok(KeyMapIdent::Key(Key {
            code: keycode,
            modification: KeyModifiers::empty(),
        }))
    }

    fn from_vecs(vec: Vec<ConfLine>) -> Result<Self> {
        let mut modes = HashMap::new();
        for ConfLine {
            command,
            mode,
            keys,
        } in vec.into_iter()
        {
            let entry = modes
                .entry(mode)
                .or_insert(HashMap::<KeyMapIdent, PreKeyMapMember<String>>::new());

            *entry = insert_recursive(entry.clone(), &keys[..], command);
        }
        let modes = modes
            .into_iter()
            .map(|(m, hm)| {
                (
                    m,
                    Arc::new(
                        hm.into_iter()
                            .map(|(k, pkm)| (k, convert_recursive(pkm)))
                            .collect::<HashMap<KeyMapIdent, KeyMapMember>>(),
                    ),
                )
            })
            .collect::<HashMap<Mode, Arc<HashMap<KeyMapIdent, KeyMapMember>>>>();
        Ok(Self {
            current_default: Mode::Normal,
            current_map: modes.get(&Mode::Normal).cloned().unwrap(),
            modes,
            nums: vec![],
        })
    }
}

fn insert_recursive<T: Clone>(
    mut map: HashMap<KeyMapIdent, PreKeyMapMember<T>>,
    keys: &[KeyMapIdent],
    value: T,
) -> HashMap<KeyMapIdent, PreKeyMapMember<T>> {
    let Some((head, tail)) = keys.split_first() else {
        return map;
    };
    if tail.is_empty() {
        map.insert(head.clone(), PreKeyMapMember::Value(value));
    } else {
        let entry = map
            .entry(head.clone())
            .or_insert(PreKeyMapMember::Map(Box::new(HashMap::new())));

        let map = match entry {
            PreKeyMapMember::Map(map) => map.clone(),
            PreKeyMapMember::Value(_) => Box::new(HashMap::new()),
        };

        *entry = PreKeyMapMember::Map(Box::new(insert_recursive(*map, tail, value)));
    }
    map
}

fn convert_recursive(map: PreKeyMapMember<String>) -> KeyMapMember {
    match map {
        PreKeyMapMember::Map(map) => KeyMapMember::Map(Arc::new(
            map.into_iter()
                .map(|(id, m)| (id, convert_recursive(m)))
                .collect::<HashMap<KeyMapIdent, KeyMapMember>>(),
        )),
        PreKeyMapMember::Value(v) => KeyMapMember::Value(v),
    }
}
