use aoc2021::{self, read_data};

#[test]
fn test_day_5() {
    let data = read_data::<String, _>("data/day06.txt").unwrap();
    let task_1 = aoc2021::day_06::day_6(&data, 80);
    assert_eq!(task_1, 359999);
    let task_2 = aoc2021::day_06::day_6(&data, 256);
    assert_eq!(task_2, 359999);
}