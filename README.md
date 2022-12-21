# Advent of Code 2022

This contains my [Advent of Code 2022](https://adventofcode.com/2022) submissions,
and is my first foray into the [Rust](https://www.rust-lang.org/) programming language.

For my unfiltered thoughts as I went through this challenge, take a look at the [Notes document](NOTES.md).

---

## Usage

To build:

```
cargo build
```

To run the examples:

Days 1-5 (divided into `a` and `b` files):
```
cargo run --bin day01a data/day01/puzzle_input.txt
cargo run --bin day01b data/day01/puzzle_input.txt
```

Days 6+ (one file for both parts):
```
cargo run --bin day06 data/day06/puzzle_input.txt
```

Also, for those puzzles that require lots of computation, you should run with the release profile:
```
cargo run -r --bin day19 data/day19/test_input.txt
```

Refer to the source files in `src/bin/` for more puzzle-specific usage and recommendations.
