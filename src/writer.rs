use crate::subs::{SubTrack};
use std::io::{prelude::*, BufWriter};
use std::fmt;

pub trait Writer {
  fn new() -> Self; 
  fn write_to(&self, path: &str, subs: &Vec<SubTrack>) -> Result<(), WriterError>;
}

pub struct WriterError;

impl fmt::Display for WriterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error writing to file. file: {}, line: {}", file!(), line!())
  }
}

pub struct CSVWRiter;

impl Writer for CSVWRiter {
  fn new() -> Self {
    CSVWRiter {}
  }

  fn write_to(&self, path: &str, subs: &Vec<SubTrack>) -> Result<(), WriterError> {
    let file = std::fs::File::create(path);

    let writer = BufWriter::new(file.unwrap());

    Ok(())
  }
}