use aoc2021::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<String, _>("data/day10.txt").unwrap();
    let task_1 = aoc2021::day_10::day_10_1(&data);
    assert_eq!(task_1, 215229);
}
