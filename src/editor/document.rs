use std::path::PathBuf;

use ropey::Rope;

pub struct Document {
    pub path: PathBuf,
    pub rope: Rope,
}
