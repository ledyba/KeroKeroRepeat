extern crate hound;

use std::error::Error;
mod pyramid;
mod sample;
use sample::Sample;
use pyramid::Layer;

pub struct Analyzer {
  source: Sample,
  root: Layer,
}

impl Analyzer {
  pub fn open(file: &str) -> Result<Analyzer, Box<dyn Error>> {
    let  source = sample::read(file)?;
    let root = pyramid::Layer::from(&source);
    return Ok(Analyzer{
      source,
      root,
    })
  }

  pub fn total_samples(&self ) -> usize {
    self.source.total_samples()
  }

  pub fn channels(&self) -> usize {
    self.source.channels()
  }

  pub fn duration(&self) -> f32 {
    self.source.duration()
  }

  pub fn calc_range(&self) {
    let size = self.source.len();
  }

  fn calc_score(&self, beg: usize, end: usize) -> f32 {
    0.0
  }
}