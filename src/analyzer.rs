extern crate hound;

use hound::WavReader;
use std::{error, io, fs};

pub(crate) struct Analyzer {
  reader: WavReader<io::BufReader<fs::File>>
}

impl Analyzer {
  pub(crate) fn open(file: &str) -> Result<Analyzer, Box<dyn error::Error>> {
    let reader = WavReader::open(file);
    if reader.is_err() {
      return Result::Err(Box::new(reader.err().unwrap()));
    }
    Result::Ok(Analyzer{
      reader: reader.unwrap()
    })
  }
}