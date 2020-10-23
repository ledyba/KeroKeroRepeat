extern crate hound;

use std::error::Error;
use self::hound::SampleFormat;

pub struct Sample {
  spec: hound::WavSpec,
  pub(crate) data: Vec<f32>,
  length: usize,
  total_samples: usize,
}

trait ToFloatSample {
  fn to_sample(&self, max: f32) -> f32;
}

impl ToFloatSample for f32 {
  fn to_sample(&self, _: f32) -> f32 {
    return *self;
  }
}

impl ToFloatSample for i32 {
  fn to_sample(&self, max: f32) -> f32 {
    return (*self as f32) / max;
  }
}

fn read_all_samples<F, T>(reader: &mut hound::WavReader<F>) -> Result<Sample, Box<dyn Error>>
  where F:std::io::Read, T: ToFloatSample + hound::Sample
{
  let spec = reader.spec();
  let mut data: Vec<f32> = Vec::new();
  let samples = reader.samples::<T>();
  let mut chan = 0;
  let channels = spec.channels as usize;
  let max = ((1 << spec.bits_per_sample) - 1) as f32;
  let mut length: usize = 0;
  let mut total_samples: usize = 0;
  for v in samples.into_iter() {
    match v {
      Result::Ok(v) => {
        total_samples = total_samples + 1;
        if chan == 0 {
          length = length + 1;
        }
        data.push((v as T).to_sample(max));
        chan = (chan + 1) % channels;
      },
      Result::Err(err) => {
        return Result::Err(Box::new(err));
      },
    }
  }
  Result::Ok(Sample{
    spec,
    data,
    length,
    total_samples,
  })
}

fn read_all<F>(reader: &mut hound::WavReader<F>) -> Result<Sample, Box<dyn Error>>
  where F:std::io::Read
{
  let fmt = reader.spec().sample_format;
  match fmt {
    SampleFormat::Float => read_all_samples::<F, f32>(reader),
    SampleFormat::Int => read_all_samples::<F, i32>(reader),
  }
}

pub fn read(file: &str) -> Result<Sample, Box<dyn Error>> {
  let opener = hound::WavReader::open(file);
  if opener.is_err() {
    return Result::Err(Box::new(opener.err().unwrap()));
  }
  let mut reader = opener.unwrap();
  read_all(&mut reader)
}

impl Sample {
  pub fn len(&self) -> usize {
    self.length
  }
  pub fn channels(&self) -> usize {
    self.spec.channels as usize
  }
  pub fn total_samples(&self) -> usize {
    self.total_samples
  }
  pub fn sample_rate(&self) -> usize {
    self.spec.sample_rate as usize
  }
  pub fn duration(&self) -> f32 {
    (self.length as f32) / (self.spec.sample_rate as f32)
  }
}