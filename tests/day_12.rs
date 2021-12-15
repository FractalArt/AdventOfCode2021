use aoc2021::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<String, _>("data/day12.txt").unwrap();
    let task_1 = aoc2021::day_12::day_12_1(&data);
    assert_eq!(task_1, 5212);
    let task_2 = aoc2021::day_12::day_12_2(&data);
    assert_eq!(task_2, 134862);
}
