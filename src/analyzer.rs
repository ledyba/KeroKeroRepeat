extern crate hound;

use std::error::Error;
mod pyramid;
mod sample;
use sample::Sample;

pub struct Analyzer {
  source: Sample,
}

impl Analyzer {
  pub fn open(file: &str) -> Result<Analyzer, Box<dyn Error>> {
    let  source = sample::read(file)?;
    return Ok(Analyzer{
      source,
    })
  }

  pub fn total_samples(&self ) -> usize {
    self.source.total_samples
  }

  pub fn channels(&self) -> usize {
    self.source.spec.channels as usize
  }

  pub fn duration(&self) -> f32 {
    (self.source.length as f32) / (self.source.spec.sample_rate as f32)
  }

  pub fn calc_range(&self) {
    let size = self.source.length;

  }

  fn calc_score(&self, beg: usize, end: usize) -> f32 {
    0.0
  }
}