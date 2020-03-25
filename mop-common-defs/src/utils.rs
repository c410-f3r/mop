#[cfg(feature = "alloc")]
use std::{
  fs::File,
  io::{self, prelude::*, BufReader},
  path::Path,
};

#[cfg(feature = "alloc")]
pub fn file_get_contents<P: AsRef<Path>>(filename: P) -> io::Result<String> {
  let file = File::open(filename)?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
  Ok(contents)
}
