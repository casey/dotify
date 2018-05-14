use std::{
  char,
  io::{self, prelude::*}
};

pub const FIRST: char = '\u{2800}';
pub const LAST:  char = '\u{28FF}';

fn byte_to_braille_cell(byte: u8) -> char {
  char::from_u32(FIRST as u32 + byte as u32).unwrap()
}

pub fn convert(reader: &mut Read, writer: &mut Write) -> io::Result<()> {
  for result in reader.bytes() {
    let byte = result?;
    let cell = byte_to_braille_cell(byte);
    write!(writer, "{}", cell)?;
  }
  Ok(())
}
