extern crate clap;

use tracing::{info, error};
use std::process::exit;

mod analyzer;

fn app() -> clap::Command {
  use clap::{value_parser, Command, Arg, ArgAction};
  Command::new("KeroKeroRepeat")
    .version("0.1.0")
    .author("Kaede Fujisaki")
    .about("Create pseudo infinite sound loops")
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .required(false)
      .action(ArgAction::Count)
      .value_parser(value_parser!(u8))
      .help("Show verbose message"))
    .arg(Arg::new("input")
      .help("input wave file")
      .short('i')
      .long("input")
      .required(true)
      .value_parser(value_parser!(String)))
    .arg(Arg::new("output")
      .help("output wave file")
      .short('o')
      .long("output")
      .value_parser(value_parser!(String)))
    .arg(Arg::new("num-workers")
      .help("number of workers")
      .long("num-workers")
      .default_value("16")
      .value_parser(value_parser!(usize)))
    .arg(Arg::new("minimum-pyramid-size")
      .help("minimum size of pyramid base")
      .long("minimum-pyramid-size")
      .default_value("1024")
      .value_parser(value_parser!(usize)))
    .arg(Arg::new("initial-search-window")
      .help("initial search window")
      .long("initial-search-window")
      .default_value("256")
      .value_parser(value_parser!(usize)))
    .arg(Arg::new("search-window")
      .help("intermediate search window")
      .long("search-window")
      .default_value("512")
      .value_parser(value_parser!(usize)))
    .arg(Arg::new("repeat-window")
      .help("repeat window")
      .long("repeat-window")
      .default_value("2048")
      .value_parser(value_parser!(usize)))
    .arg(Arg::new("repeat-count")
      .help("repeat window")
      .long("repeat-count")
      .short('c')
      .default_value("10")
      .value_parser(value_parser!(usize)))
}

fn main() {
  use tracing_subscriber::util::SubscriberInitExt;
  let app = app();
  let m = app.get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => tracing::Level::INFO,
    Some(1) => tracing::Level::DEBUG,
    _ => tracing::Level::TRACE,
  };
  tracing_subscriber::fmt()
    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new("%Y/%m/%d %H:%M:%S%.3f".to_string()))
    .with_max_level(log_level)
    .with_line_number(true)
    .with_file(true)
    .with_writer(std::io::stderr)
    .finish()
    .init();
  let input = m.get_one::<String>("input").unwrap();
  let output = m.get_one::<String>("output").unwrap();
  info!("KeroKero: {} -> {}\n", input, output);
  if !std::path::Path::new(&input).exists() {
    error!("File not found: {}\n", input);
    exit(-1);
  }
  let num_workers = *m.get_one::<usize>("num-workers").expect("[BUG] No num-workers");
  let minimum_pyramid_size = *m.get_one::<usize>("minimum-pyramid-size").expect("[BUG] No minimum-pyramid-size");
  let analyzer = analyzer::Analyzer::open(input, num_workers, minimum_pyramid_size);
  if analyzer.is_err() {
    error!("Failed to open input: {}\n", analyzer.err().unwrap().to_string());
    exit(-1);
  }
  let analyzer = analyzer.unwrap();
  let initial_search_window = *m.get_one::<usize>("initial-search-window").expect("[BUG] No initial-search-window");
  let search_window = *m.get_one::<usize>("search-window").expect("[BUG] No search-window");
  info!("Loaded {} samples in {} channels ({:.2} sec)", analyzer.total_samples(), analyzer.channels(), analyzer.duration());
  let result = analyzer.calc_root(initial_search_window);
  let root_level = analyzer.root_level();
  info!("level={} len={} range=({}, {}) score={:.5}", root_level, result.3, result.0, result.1, result.2);
  let mut i = result.0;
  let mut j = result.1;
  for level in 1..root_level {
    let result = analyzer.calc_layer(search_window, i*2+1, j*2+1, root_level - level);
    i = result.0;
    j = result.1;
    info!("level={} len={} range=({}, {}) score={:.5}", root_level - level, result.3, result.0, result.1, result.2);
  }
  let repeat_window = *m.get_one::<usize>("repeat-window").expect("[BUG] No repeat-window");
  let repeat_count = *m.get_one::<usize>("repeat-count").expect("[BUG] No repeat-count");
  let result = analyzer.source.write_back(output, i as usize, j, repeat_window, repeat_count);
  if result.is_err() {
    error!("Failed to open input: {:?}\n", result.unwrap_err());
    exit(-1);
  }
  {
    let beg = analyzer.source.time_at(i);
    let end = analyzer.source.time_at(j);
    info!("Detected repeat range: {:.2} -> {:.2} ({:.2} sec)", beg, end, end-beg);
  }
  let written = result.unwrap();
  info!("Written {} samples in {} channels ({:.2} sec)", written, analyzer.channels(), written as f64 / analyzer.source.sample_rate() as f64);
}
