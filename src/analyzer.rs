extern crate hound;

use std::error::Error;
mod pyramid;
mod sample;
use sample::Sample;
use pyramid::Layer;
use std::cmp::{max, min};

pub struct Analyzer {
  pub(crate) source: Sample,
  pyramid: Vec<Layer>,
}

impl Analyzer {
  pub fn open(file: &str, minimum_pyramid_size: usize) -> Result<Analyzer, Box<dyn Error>> {
    let  source = sample::read(file)?;
    let mut pyramid: Vec<Layer> = vec![];
    pyramid.push(pyramid::Layer::from(&source));
    while pyramid[pyramid.len()-1].data.len() > minimum_pyramid_size {
      pyramid.push(pyramid[pyramid.len()-1].next());
    }
    return Ok(Analyzer{
      source,
      pyramid,
    })
  }

  pub fn total_samples(&self ) -> usize {
    self.source.total_samples()
  }

  pub fn channels(&self) -> usize {
    self.source.channels()
  }

  pub fn duration(&self) -> f64 {
    self.source.duration()
  }

  pub fn root_level(&self) -> usize {
    self.pyramid.len() - 1
  }

  fn calc_sum(data: &Vec<f32>, i: usize, j:usize, width: usize) -> f64 {
    let mut sum:f64 = 0.0;
    let mut sum_i:f64 = 0.0;
    let mut sum_j:f64 = 0.0;
    for k in 0..width {
      let vi = data[i + k] as f64;
      let vj = data[j + k] as f64;
      sum = sum + vi * vj;
      sum_i = sum_i + (vi * vi);
      sum_j = sum_j + (vj * vj);
    }
    sum / (sum_i * sum_j).sqrt()
  }

  pub fn calc_root(&self, width: usize) -> (usize, usize, f64) {
    let mut max_sum:f64 = std::f64::NEG_INFINITY;
    let mut max_idx:(usize, usize) = (0, 0);
    let layer = &self.pyramid[self.root_level()];
    let layer_len = layer.data.len();
    for i in 0..layer_len-width {
      for j in i+width*2..layer_len-width {
        let sum = Analyzer::calc_sum(&layer.data, i, j, width);
        if sum > max_sum {
          max_idx = (i, j);
          max_sum = sum;
        }
      }
    }
    let (i,j) = max_idx;
    (i, j, max_sum / width as f64)
  }

  pub fn calc_next(&self, width: usize, fi: usize, fj: usize, level: usize) -> (usize, usize, f64) {
    let layer = &self.pyramid[level - 1];

    let mut max_sum:f64 = std::f64::NEG_INFINITY;
    let mut max_idx:(usize, usize) = (0, 0);
    let layer_len = layer.data.len();
    for i in max(width, fi*2)-width..min(fi*2+width, layer_len - width) {
      for j in max(width, fj*2)-width..min(fj*2+width, layer_len - width) {
        if ((j as isize - i as isize).abs() as usize) < width*2 {
          continue;
        }
        let sum = Analyzer::calc_sum(&layer.data, i, j, width);
        if sum > max_sum {
          max_idx = (i, j);
          max_sum = sum;
        }
      }
    }
    let (i,j) = max_idx;
    (i, j, max_sum)
  }
}