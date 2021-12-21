# Advent of Code 2021

My solutions to the [AOC 2021](https://adventofcode.com/2021) problems in [Rust](https://www.rust-lang.org/).

## Solutions

| Task | Status |
| ---- | :----: |
| Day 1 | :heavy_check_mark:, :heavy_check_mark: |
| Day 2 | :heavy_check_mark:, :heavy_check_mark: |
| Day 3 | :heavy_check_mark:, :heavy_check_mark: |
| Day 4 | :heavy_check_mark:, :heavy_check_mark: |
| Day 5 | :heavy_check_mark:, :heavy_check_mark: |
| Day 6 | :heavy_check_mark:, :heavy_check_mark: |
| Day 7 | :heavy_check_mark:, :heavy_check_mark: |
| Day 8 | :heavy_check_mark:, :heavy_check_mark: |
| Day 9 | :heavy_check_mark:, :heavy_check_mark: |
| Day 10 | :heavy_check_mark:, :heavy_check_mark: |
| Day 11 | :heavy_check_mark:, :heavy_check_mark: |
| Day 12 | :heavy_check_mark:, :heavy_check_mark: |
| Day 13 | :heavy_check_mark:, :heavy_check_mark: |
| Day 14 | :heavy_check_mark:, :heavy_check_mark: |
| Day 15 | :x:, :x: |
| Day 16 | :x:, :x: |
| Day 17 | :x:, :x: |
| Day 18 | :x:, :x: |
| Day 19 | :x:, :x: |
| Day 20 | :x:, :x: |
| Day 21 | :x:, :x: |

***

## Organization

Each day's solutions are implemented in a separate module such as `day_01.rs`. This module usually contains the examples that explain the problem as unit tests.

For each day, there is an integration test, named for example `day_01.rs` in the `tests` subdirectory which makes sure that the functionality in the different modules produce the correct solutions when applied to the provided input files.

To run the tests for a specific day, run for example

```sh
cargo t --release --test day_01
```
