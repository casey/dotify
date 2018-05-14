use std::{
  char,
  io::{self, prelude::*},
};

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let stdout = io::stdout();
  let mut reader = stdin.lock();
  let mut writer = stdout.lock();

  let mut braille = String::new();

  loop {
    let bytes_read = {
      let buffer = reader.fill_buf()?;

      if buffer.is_empty() {
        return Ok(());
      }

      braille.clear();

      for byte in buffer {
        let cell = char::from_u32(0x2800 + *byte as u32).unwrap();
        braille.push(cell);
      }

      buffer.len()
    };

    writer.write(braille.as_bytes())?;

    reader.consume(bytes_read);
  }
}
