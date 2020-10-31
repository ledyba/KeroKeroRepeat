extern crate hound;

use std::error::Error;
mod pyramid;
mod sample;
use sample::Sample;
use pyramid::Layer;
use std::cmp::{max, min};
use log::{error};
use std::sync::Arc;

pub struct Analyzer {
  pub(crate) source: Sample,
  num_workers: usize,
  pyramid: Vec<Arc<Layer>>,
}

impl Analyzer {
  pub fn open(file: &str, num_workers: usize, minimum_pyramid_size: usize) -> Result<Analyzer, Box<dyn Error>> {
    let  source = sample::read(file)?;
    let mut pyramid: Vec<Arc<Layer>> = vec![];
    pyramid.push(Arc::new(pyramid::Layer::from(&source)));
    while pyramid[pyramid.len()-1].data.len() > minimum_pyramid_size {
      pyramid.push(Arc::new(pyramid[pyramid.len()-1].next()));
    }
    return Ok(Analyzer{
      source,
      num_workers,
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
    self.pyramid.len()
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

  pub fn calc_root(&self, width: usize) -> (usize, usize, f64, usize) {
    self.calc_layer(width, 0, width, self.root_level())
  }

  pub fn calc_layer(&self, width: usize, fi: usize, fj: usize, level: usize) -> (usize, usize, f64, usize) {
    let layer = self.pyramid[level - 1].clone();
    let layer_len = layer.data.len();
    let rt = tokio::runtime::Builder::new_multi_thread().max_threads(self.num_workers).build();
    if rt.is_err() {
      error!("Failed to initialize runtime: {:?}", rt.unwrap_err());
      std::process::exit(-1);
    }
    let rt = rt.unwrap();
    rt.block_on(async {
      let mut max_result: (usize, usize, f64) = (0, 0, std::f64::NEG_INFINITY);
      let mut sums = vec![];
      for i in fi..layer_len-width {
        let layer = layer.clone();
        sums.push(rt.spawn(async move {
          let mut max_result: (usize, usize, f64) = (0, 0, std::f64::NEG_INFINITY);
          for j in max(fj, i+width*2)..layer_len-width {
            if ((j as isize - i as isize).abs() as usize) < width * 2 {
              continue;
            }
            let score = Analyzer::calc_sum(&layer.data, i, j, width);
            if max_result.2 < score {
              max_result = (i, j, score);
            }
          }
          max_result
        }));
      }
      for result in futures::future::join_all(sums.into_iter()).await.into_iter() {
        let (i, j, score) = result.unwrap();
        if max_result.2 < score {
          max_result = (i, j, score);
        }
      }
      (max_result.0, max_result.1, max_result.2, layer_len)
    })
  }
}