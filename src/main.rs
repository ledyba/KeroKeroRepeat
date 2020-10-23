extern crate clap;

use log::{info, error};

use clap::{App, Arg};
use std::process::exit;

mod analyzer;

fn main() {
  env_logger::init_from_env(
    env_logger::Env::from(env_logger::Env::default())
      .default_filter_or("info"));
  fn is_valid_ext(v: String) -> Result<(), String> {
    if v.ends_with(".wav") {
      return Result::Ok(())
    }
    return Result::Err(format!("Should be wav file: {}", v))
  }
  fn is_number(v: String) -> Result<(), String> {
    v.parse::<usize>().map(|_| ()).map_err(|err| err.to_string())
  }
  let app = App::new("KeroKeroRepeat")
    .version("0.1.0")
    .author("Kaede Fujisaki")
    .about("Create pseudo infinite sound loops")
    .arg(Arg::with_name("input")
      .help("input wave file")
      .short("i")
      .long("input")
      .required(true)
      .takes_value(true)
      .validator(is_valid_ext))
    .arg(Arg::with_name("output")
      .help("output wave file")
      .short("o")
      .long("output")
      .required(true)
      .takes_value(true)
      .validator(is_valid_ext))
    .arg(Arg::with_name("minimum-pyramid-size")
      .help("initial search window")
      .long("minimum-pyramid-size")
      .default_value("1024")
      .required(false)
      .takes_value(true)
      .validator(is_number))
    .arg(Arg::with_name("initial-search-window")
      .help("initial search window")
      .long("initial-search-window")
      .default_value("256")
      .required(false)
      .takes_value(true)
      .validator(is_number))
    .arg(Arg::with_name("search-window")
      .help("intermediate search window")
      .long("search-window")
      .default_value("512")
      .required(false)
      .takes_value(true)
      .validator(is_number))
    .arg(Arg::with_name("repeat-window")
      .help("repeat window")
      .long("repeat-window")
      .default_value("2048")
      .required(false)
      .takes_value(true)
      .validator(is_number))
    .arg(Arg::with_name("repeat-count")
      .help("repeat window")
      .long("repeat-count")
      .default_value("10")
      .required(false)
      .takes_value(true)
      .validator(is_number));
  let matches = app.get_matches();
  let input = matches.value_of("input").unwrap();
  let output = matches.value_of("output").unwrap();
  info!("KeroKero: {} -> {}\n", input, output);
  if !std::path::Path::new(&input).exists() {
    error!("File not found: {}\n", input);
    exit(-1);
  }
  let minimum_pyramid_size = matches.value_of("minimum-pyramid-size").unwrap().parse::<usize>().unwrap();
  let analyzer = analyzer::Analyzer::open(input, minimum_pyramid_size);
  if analyzer.is_err() {
    error!("Failed to open input: {}\n", analyzer.err().unwrap().to_string());
    exit(-1);
  }
  let analyzer = analyzer.unwrap();
  let initial_search_window = matches.value_of("initial-search-window").unwrap().parse::<usize>().unwrap();
  let search_window = matches.value_of("search-window").unwrap().parse::<usize>().unwrap();
  info!("Loaded {} samples in {} channels ({} sec)", analyzer.total_samples(), analyzer.channels(), analyzer.duration());
  let (mut i,mut j,score) = analyzer.calc_root(initial_search_window);
  let root_level = analyzer.root_level();
  info!("level={} ({}, {}) score={:.5}", root_level, i, j, score);
  for level in 0..root_level {
    let ij = analyzer.calc_next(search_window, i, j, root_level - level);
    i = ij.0;
    j = ij.1;
    let score = ij.2;
    info!("level={} ({}, {}) score={:.5}", root_level - level - 1, i, j, score);
  }
  let repeat_window = matches.value_of("repeat-window").unwrap().parse::<usize>().unwrap();
  let repeat_count = matches.value_of("repeat-count").unwrap().parse::<usize>().unwrap();
  let result = analyzer.source.write_back(output, i, j, repeat_window, repeat_count);
  if result.is_err() {
    error!("Failed to open input: {:?}\n", result.unwrap_err());
    exit(-1);
  }
  {
    let beg = analyzer.source.time_at(i);
    let end = analyzer.source.time_at(j);
    info!("Detected repeat range: {} -> {} ({:.2} sec)", beg, end, end-beg);
  }
  let written = result.unwrap();
  info!("Written {} samples in {} channels ({:.2} sec)", written, analyzer.channels(), written as f64 / analyzer.source.sample_rate() as f64);
}
