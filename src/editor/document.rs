use std::{io::Write, path::PathBuf};

use ropey::Rope;

pub struct Document {
    pub path: PathBuf,
    pub rope: Rope,
}

impl Document {
    pub fn write(&self) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(&self.path)?;
        file.write_all(self.rope.to_string().as_bytes())?;
        Ok(())
    }
}
