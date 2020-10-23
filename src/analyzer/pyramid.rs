use crate::analyzer::sample::Sample;

pub struct  Layer {
  pub level: usize,
  pub data: Vec<f32>,
}

impl Layer {
  pub fn from(sample: &Sample) -> Layer {
    let mut data = Vec::<f32>::new();
    let mut sum:f64 = 0.0;
    for idx in 0..sample.total_samples() {
      if (idx % sample.channels()) == 0 {
        if idx > 0 {
          data.push(sum as f32);
        }
        sum = 0.0;
      }
      sum = sum + (sample.data[idx] as f64);
    }
    Layer {
      level: 1,
      data,
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }
  pub fn next(&self) -> Layer {
    let mut data = Vec::<f32>::new();
    let mut sum: f32 = 0.0;
    for i in 0..self.data.len() {
      if i % 2 == 0 {
        if i > 0 {
          data.push(sum / 2.0);
        }
        sum = 0.0;
      }
      sum = sum + self.data[i];
    }
    Layer {
      level: self.level + 1,
      data,
    }
  }
}