use aoc2021::{self, read_data};
use itertools::Itertools;

#[test]
fn test_day_13() {
    let data = read_data::<String, _>("data/day13.txt").unwrap();
    let task_1 = aoc2021::day_13::day_13_1(&data);
    assert_eq!(task_1, 607);

    let target = vec![
        ".##..###..####.#....###..####.####.#...",
        "#..#.#..#....#.#....#..#.#.......#.#...",
        "#....#..#...#..#....#..#.###....#..#...",
        "#....###...#...#....###..#.....#...#...",
        "#..#.#....#....#....#....#....#....#...",
        ".##..#....####.####.#....#....####.####",
    ]
    .into_iter()
    .join("\n");

    let task_2 = aoc2021::day_13::day_13_2(&data);
    assert_eq!(task_2, target);

    // The solution to the puzzle is: CPZLPFZL
}
