extern crate hound;

use hound::{WavReader, WavSpec, Sample};
use std::error::Error;
use self::hound::SampleFormat;
use std::borrow::BorrowMut;

pub struct Analyzer {
  spec: WavSpec,
  data: Vec<Vec<f32>>,
  width: usize,
}

trait ToFloatSample {
  fn to_sample(&self, max: f32) -> f32;
}

impl ToFloatSample for f32 {
  fn to_sample(&self, _: f32) -> f32 {
    return *self
  }
}

impl ToFloatSample for i32 {
  fn to_sample(&self, max: f32) -> f32 {
    return (*self as f32) / max;
  }
}

fn read_all_samples<F, T>(reader: &mut WavReader<F>) -> Result<Vec<Vec<f32>>, Box<dyn Error>>
  where F:std::io::Read, T: ToFloatSample + Sample
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
  pub fn open(file: &str, width: f32) -> Result<Analyzer, Box<dyn Error>> {
    let opener = WavReader::open(file);
    if opener.is_err() {
      return Result::Err(Box::new(opener.err().unwrap()));
    }
    let mut reader = opener.unwrap();
    let spec = reader.spec();
    let data = read_all(reader.borrow_mut());
    let width = ((spec.sample_rate as f32) * width) as usize;
    data.map(|data| {
      Analyzer{
        spec,
        data,
        width,
      }
    })
  }
  pub fn total_samples(&self) -> usize {
    (&self.data).into_iter().fold(0, |sum, ch| sum + ch.len())
  }
  pub fn channels(&self) -> usize {
    self.spec.channels as usize
  }

  pub fn duration(&self) -> f32 {
    (self.data[0].len() as f32) / (self.spec.sample_rate as f32)
  }

  pub fn calc_range(&self) {
    let size = self.data[0].len();
    for i in (self.width..(size - self.width)).step_by(self.width/100) {
      for j in (self.width..(size - self.width)).step_by(self.width/100) {
        let score = self.calc_score(i, j);
        if score > 0.1 {
          print!("{} -> {}", i, j);
        }
      }
    }
  }

  fn calc_score(&self, beg: usize, end: usize) -> f32 {
    let mut total_score = 1.0 as f32;
    for ch in 0..(self.spec.channels as usize) {
      let mut ch_score:f32 = 0.0;
      let buff = &self.data[ch];
      for i in (-(self.width as isize)..(self.width as isize)).step_by(self.width/1000) {
        for j in (-(self.width as isize)..(self.width as isize)).step_by(self.width/1000) {
          ch_score += buff[(beg as isize + i) as usize] * buff[(end as isize + j) as usize];
        }
      }
      if ch_score < total_score {
        total_score = ch_score;
      }
    }
    total_score
  }
}