use aoc2021::{self, read_data};

#[test]
fn test_day_4() {
    let data = read_data::<String, _>("data/day04.txt").unwrap();
    let task_1 = aoc2021::day_04::day_4_1(&data);
    assert_eq!(task_1, 39902);
    let task_2 = aoc2021::day_04::day_4_2(&data);
    assert_eq!(task_2, 26936);
}
