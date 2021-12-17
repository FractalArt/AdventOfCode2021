use aoc2021::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<String, _>("data/day14.txt").unwrap();
    let task_1 = aoc2021::day_14::day_14_1(&data, 10);
    assert_eq!(task_1, 5656);
}
