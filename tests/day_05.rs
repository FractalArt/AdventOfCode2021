use aoc2021::{self, read_data};

#[test]
fn test_day_5() {
    let data = read_data::<String, _>("data/day05.txt").unwrap();
    let task_1 = aoc2021::day_05::day_5_1(&data);
    assert_eq!(task_1, 5124);
}
