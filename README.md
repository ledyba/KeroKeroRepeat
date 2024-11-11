# KeroKero🐸Repeat （ケロケロ🐸リピート）

[![Build on Linux](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20on%20Linux/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+on+Linux%22)
[![Build on macOS](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20on%20macOS/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+on+macOS%22)
[![Build on Windows](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20on%20Windows/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+on+Windows%22)  
[![Build single binary on Linux](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20single%20binary%20on%20Linux/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+single+binary+on+Linux%22)
[![Generate example files](https://github.com/ledyba/KeroKeroRepeat/workflows/Generate%20example%20files/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Generate+example+files%22)

A software to create multiple extended loops from an audio file. 

与えられたオーディオファイルの一部分を不自然でないように何度もループさせたファイル作成するソフト。

## Demo

 - Input: [input.wav](https://github.com/ledyba/KeroKeroRepeat/raw/magistra/input.wav)
 - Output: [input.wav](https://github.com/ledyba/KeroKeroRepeat/raw/magistra/output.ogg)

In output.wav, 6.14 sec to 53.83 sec (47.69 sec) in input.wav repeats 3 times.

### Use single binary

### Build with Cargo

```bash
cargo build --release
```

then run,

```bash
% target/debug/KeroKeroRepeat --help
Create pseudo infinite sound loops

Usage: KeroKeroRepeat.exe [OPTIONS] --input <input>

Options:
  -v, --verbose...                                     Show verbose message
  -i, --input <input>                                  input wave file
  -o, --output <output>                                output wave file
      --num-workers <num-workers>                      number of workers [default: 16]
      --minimum-pyramid-size <minimum-pyramid-size>    minimum size of pyramid base [default: 1024]
      --initial-search-window <initial-search-window>  initial search window [default: 256]
      --search-window <search-window>                  intermediate search window [default: 512]
      --repeat-window <repeat-window>                  repeat window [default: 2048]
  -c, --repeat-count <repeat-count>                    repeat window [default: 10]
  -h, --help                                           Print help
  -V, --version                                        Print version
```

# Why 'KeroKeroRepeat'?

Please listen input file or result file.
