use std::io::BufRead;

use anyhow::Context;

pub(super) fn io_read(reader: &mut impl BufRead) -> anyhow::Result<Option<String>> {
    let mut content_lenght = None;
    let mut buf = String::new();
    loop {
        buf.clear();
        let bytes_count = reader.read_line(&mut buf)?;
        if bytes_count == 0 {
            return Ok(None);
        }
        if buf == "\n" || buf == "\r\n" {
            break;
        }
        if let Some((name, value)) = buf.split_once(":")
            && name.trim().eq_ignore_ascii_case("Content-Length")
        {
            let len = value.trim().parse::<usize>()?;
            content_lenght = Some(len);
        }
    }

    let len = content_lenght.context("No Length in header")?;

    let mut body = vec![0u8; len];

    reader.read_exact(&mut body)?;

    let payload = String::from_utf8(body)?;

    Ok(Some(payload))
}
