use std::{
  str,
  io::{self, prelude::*},
};

use braille::{FIRST, LAST};

fn braille_cell_to_byte(cell: char) -> Option<u8> {
  if cell < FIRST || cell > LAST {
    None
  } else {
    Some((cell as u32 - 0x2800) as u8)
  }
}

pub fn convert(reader: &mut Read, writer: &mut Write) -> io::Result<()> {
  let mut buffer = Vec::new();
  for result in reader.bytes() {
    let byte = result?;
    buffer.push(byte);
    let clear = match str::from_utf8(&buffer) {
      Ok(s) => {
        for c in s.chars() {
          if let Some(byte) = braille_cell_to_byte(c) {
            writer.write(&[byte])?;
          }
        };
        true
      }
      Err(utf8_error) => utf8_error.error_len().is_some(),
    };

    if clear {
      buffer.clear();
    }
  }
  Ok(())
}
