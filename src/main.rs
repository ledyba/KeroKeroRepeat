extern crate clap;

use clap::{App, Arg};
use std::error::Error;

mod analyzer;

fn main() {
  fn is_valid_ext(v: String) -> Result<(), String> {
    if v.ends_with(".wav") {
      return Result::Ok(())
    }
    return Result::Err(format!("Should be wav file: {}", v))
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
      .validator(is_valid_ext));
  let matches = app.get_matches();
  let input = matches.value_of("input").unwrap();
  let output = matches.value_of("input").unwrap();
  print!("{} -> {}\n", input, output);
  let analyzer = analyzer::Analyzer::open(input);
  if analyzer.is_err() {
    print!("Failed to open input: {}\n", analyzer.err().unwrap().to_string())
  }
}
