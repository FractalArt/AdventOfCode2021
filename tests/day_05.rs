use aoc2021::{self, read_data};

#[test]
fn test_day_5() {
    let data = read_data::<String, _>("data/day05.txt").unwrap();
    let task_1 = aoc2021::day_05::day_5(&data, false);
    assert_eq!(task_1, 5124);
    let task_2 = aoc2021::day_05::day_5(&data, true);
    assert_eq!(task_2, 19771);
}
