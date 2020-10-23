# KeroKeroRepeat （ケロ🐸ケロ🐸リピート）

[![Build on Linux](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20on%20Linux/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+on+Linux%22)
[![Build on macOS](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20on%20macOS/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+on+macOS%22)
[![Build on Windows](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20on%20Windows/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+on+Windows%22)  
[![Build single binary on Linux](https://github.com/ledyba/KeroKeroRepeat/workflows/Build%20single%20binary%20on%20Linux/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Build+single+binary+on+Linux%22)
[![Generate example files](https://github.com/ledyba/KeroKeroRepeat/workflows/Generate%20example%20files/badge.svg)](https://github.com/ledyba/KeroKeroRepeat/actions?query=workflow%3A%22Generate+example+files%22)

A software to create endless-gif-loops from animation video files.

与えられたwavファイルを解析して、つなぎ合わせてリピートしても不自然でない部分をつなげてリピートしたwavファイルを出力するソフトウェア。

## Demo

 - Input: [input.wav](https://github.com/ledyba/KeroKeroRepeat/raw/magistra/input.wav)
 - Output: [input.wav](https://github.com/ledyba/KeroKeroRepeat/raw/magistra/output.wav)

In output.wav, 6.137sec to 53.825sec in input.wav repeats 3 times.

### Use single binary

### Build with Cargo

```bash
cargo build --release
```

then run,

```bash
% target/debug/KeroKeroRepeat --help
KeroKeroRepeat 0.1.0
Kaede Fujisaki
Create pseudo infinite sound loops

USAGE:
    KeroKeroRepeat [OPTIONS] --input <input> --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --initial-search-window <initial-search-window>    initial search window [default: 256]
    -i, --input <input>                                    input wave file
        --minimum-pyramid-size <minimum-pyramid-size>      initial search window [default: 1024]
    -o, --output <output>                                  output wave file
    -c, --repeat-count <repeat-count>                      repeat window [default: 10]
        --repeat-window <repeat-window>                    repeat window [default: 2048]
        --search-window <search-window>                    intermediate search window [default: 512]
```
