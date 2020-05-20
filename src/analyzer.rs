extern crate hound;

use hound::{WavReader, WavSpec, Sample};
use std::error::Error;
use self::hound::SampleFormat;
use std::borrow::BorrowMut;

pub struct Analyzer {
  spec: WavSpec,
  data: Vec<Vec<f32>>
}

trait ToSample {
  fn to_sample(&self, max: f32) -> f32;
}

impl ToSample for f32 {
  fn to_sample(&self, _: f32) -> f32 {
    return *self
  }
}

impl ToSample for i32 {
  fn to_sample(&self, max: f32) -> f32 {
    return (*self as f32) / max;
  }
}

fn read_all_samples<F, T>(reader: &mut WavReader<F>) -> Result<Vec<Vec<f32>>, Box<dyn Error>>
  where F:std::io::Read, T: ToSample + Sample
{
  let spec = reader.spec();
  let mut data: Vec<Vec<f32>> = Vec::new();
  let iter = reader.samples::<T>();
  let mut chan = 0;
  let channels = spec.channels as usize;
  for _ in 0..channels {
    data.push(Vec::new());
  }
  let max = (1 << spec.bits_per_sample) as f32;
  for v in iter {
    match v {
      Result::Ok(v) => {
        data[chan].push((v as T).to_sample(max));
        chan = (chan + 1) % channels;
      },
      Result::Err(err) => {
        return Result::Err(Box::new(err));
      },
    }
  }
  Result::Ok(data)
}

fn read_all<F>(reader: &mut WavReader<F>) -> Result<Vec<Vec<f32>>, Box<dyn Error>>
  where F:std::io::Read
{
  let fmt = reader.spec().sample_format;
  match fmt {
    SampleFormat::Float => read_all_samples::<F, f32>(reader),
    SampleFormat::Int => read_all_samples::<F, i32>(reader),
  }
}

impl Analyzer {
  pub(crate) fn open(file: &str) -> Result<Analyzer, Box<dyn Error>> {
    let opener = WavReader::open(file);
    if opener.is_err() {
      return Result::Err(Box::new(opener.err().unwrap()));
    }
    let mut reader = opener.unwrap();
    let spec = reader.spec().into();
    let data = read_all(reader.borrow_mut());
    data.map(|data| {
      Analyzer{
        spec,
        data
      }
    })
  }
}