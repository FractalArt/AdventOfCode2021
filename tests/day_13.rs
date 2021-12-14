use aoc2021::{self, read_data};

#[test]
fn test_day_13() {
    let data = read_data::<String, _>("data/day13.txt").unwrap();
    let task_1 = aoc2021::day_13::day_13_1(&data);
    assert_eq!(task_1, 607);
}
