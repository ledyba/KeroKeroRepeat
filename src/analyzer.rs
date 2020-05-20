extern crate hound;

use hound::{WavReader, WavSpec, WavSamples};
use std::{error, io, fs};

pub struct Analyzer {
  spec: WavSpec,
  data: Vec<Vec<f32>>
}

impl Analyzer {
  pub(crate) fn open(file: &str) -> Result<Analyzer, Box<dyn error::Error>> {
    let opener = WavReader::open(file);
    if opener.is_err() {
      return Result::Err(Box::new(opener.err().unwrap()));
    }
    let mut reader = opener.unwrap();
    let spec = reader.spec();
    let mut data: Vec<Vec<f32>> = Vec::new();
    let iter = reader.samples::<f32>();
    let mut chan = 0;
    for _ in 0..spec.channels {
      data.push(Vec::new());
    }
    for v in iter {
      if v.is_err() {
        return Result::Err(Box::new(v.err().unwrap()));
      }
      data[chan].push(v.unwrap());
      chan = (chan + 1) % (spec.channels as usize);
    }
    Result::Ok(Analyzer{
      spec,
      data
    })
  }
}