use aoc2021::{self, read_data};

#[test]
fn test_day_8() {
    let data = read_data::<String, _>("data/day08.txt").unwrap();
    let task_1 = aoc2021::day_08::day_8_1(&data);
    assert_eq!(task_1, 440);
}
