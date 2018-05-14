extern crate clap;

mod braille;
mod bytes;

use clap::{App, Arg, AppSettings};
use std::io;

fn main() -> io::Result<()> {
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(env!("CARGO_PKG_DESCRIPTION"), " - ", env!("CARGO_PKG_HOMEPAGE")))
    .help_message("Print help information")
    .version_message("Print version information")
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("BYTES")
         .long("bytes")
         .help("Convert braille to bytes"))
    .get_matches();

  let bytes = matches.is_present("BYTES");

  let stdin = io::stdin();
  let stdout = io::stdout();
  let mut reader = stdin.lock();
  let mut writer = stdout.lock();

  let convert = if bytes {
    bytes::convert
  } else {
    braille::convert
  };

  convert(&mut reader, &mut writer)
}
